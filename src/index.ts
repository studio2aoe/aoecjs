const SAMPLE_BUFSIZE = 128

interface AoecExports extends WebAssembly.Exports {
  outbuf_alloc: CallableFunction
  set_sample_rate: CallableFunction
  set_freq: CallableFunction
  set_vol: CallableFunction
  set_mute: CallableFunction
  set_param: CallableFunction
  process: CallableFunction
  memory: WebAssembly.Memory
}

class SoundChip extends AudioWorkletProcessor {
  _wasmExports!: AoecExports
  _outbuf!: [Float32Array, Float32Array]
  _outptr!: [number, number]
  
  constructor () {
    super()
    this.setupMessageHandler()
  }

  setupMessageHandler () {
    this.port.onmessage = async e => {

      // The wasm module can't be loaded directly in worklet scope,
      // because `fetch()` isn't work in the scope.
      // so the wasm is loaded at main window and sent as message.
      const msg = e.data
      if (msg.type === 'loadWasm') {
        await this.initWasm(msg.data)
        this.port.postMessage({ type: 'wasmReady' })
      }

      if (msg.type === 'wasmfunc') {
        if (!this._wasmExports) return true
        // console.log(msg.data.name + ' (' + msg.data.arg + ')')
        switch (msg.data.name) {
          case 'set_freq':
            this._wasmExports.set_freq(
              msg.data.arg[0], // id (u64 -> usize)
              msg.data.arg[1] //  frequency (f32)
            )
            break
          case 'set_vol':
            this._wasmExports.set_vol(
              msg.data.arg[0], // id (u64 -> usize)
              msg.data.arg[1], // channel (u64 -> usize)
              msg.data.arg[2] //  volume (u64 -> u8)
            )
            break
          case 'set_mute':
            this._wasmExports.set_mute(
              msg.data.arg[0], // id (u64 -> usize)
              msg.data.arg[1] //  mute (u64 -> bool)
            )
            break
          case 'set_param':
            this._wasmExports.set_param(
              msg.data.arg[0], // id (u64 -> usize)
              msg.data.arg[1], // key (u64 -> usize)
              msg.data.arg[2] //  value (u64 -> u32)
            )
            break
        }
      }
    }
  }

  async initWasm (data: BufferSource) {    
    /* I don't know why the `importObject` is needed,
    * because rust crate doesn't use any function from JS.
    * but compiled wasm has imported "$now" function at first line.
    * `(func $now (;0;) (import "env" "now") (result f64))`
    * so I send the empty function `env.now` in `importObject`.
    */
    const importObject = { env: { now: () => {} } }
    this._wasmExports = await WebAssembly.instantiate(data, importObject)
      .then(w => w.instance.exports as AoecExports)

    this._wasmExports.set_sample_rate(sampleRate)

    this._outptr = [
      this._wasmExports.outbuf_alloc(SAMPLE_BUFSIZE),
      this._wasmExports.outbuf_alloc(SAMPLE_BUFSIZE)
    ]

    this._outbuf = [
      new Float32Array(
        this._wasmExports.memory.buffer,
        this._outptr[0],
        SAMPLE_BUFSIZE
      ),
      new Float32Array(
        this._wasmExports.memory.buffer,
        this._outptr[1],
        SAMPLE_BUFSIZE
      )
    ]
  }

  process (
    inputs: Float32Array[][], 
    outputs: Float32Array[][], 
    params: any) {
    if (!this._wasmExports) {
      return true
    }

    // actual process in wasm module: see crate/src/lib.rs
    this._wasmExports.process(
      this._outptr[0],
      this._outptr[1],
      currentFrame
    )

    // Sync the output buffer
    for (let num = 0; num < outputs.length; num++) {
      outputs[num][0].set(this._outbuf[0])
      outputs[num][1].set(this._outbuf[1])
    }

    return true
  }
}

registerProcessor('aoecProcessor', SoundChip)
