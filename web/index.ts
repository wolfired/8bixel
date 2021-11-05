class MsgKeyboard {
  private static ID: number = 10;

  constructor(public key: number, public status: number) {}

  public write(writer: IOHelper): void {
    let dat_view = new DataView(new ArrayBuffer(8));

    let offset = 0;

    dat_view.setUint32(offset, this.key, true);
    dat_view.setUint32((offset += 4), this.status, true);

    writer.write(MsgKeyboard.ID, dat_view.buffer);
  }
}

class IOHelper {
  private cursor_r: number = 0;

  private skipped_count_r: number = 0;
  private skipped_count_w: number = 0;

  private total_count_r: number = 0;
  private total_count_w: number = 0;

  constructor(public buf: DataView, private cursor_w_buf: DataView) {}

  private get cursor_w(): number {
    return this.cursor_w_buf.getUint32(0, true);
  }

  private set cursor_w(value: number) {
    this.cursor_w_buf.setUint32(0, value, true);
  }

  public read(): boolean {
    let cursor_r = this.cursor_r % this.buf.byteLength;
    let gap_r = this.buf.byteLength - cursor_r;

    if (8 > gap_r) {
      this.cursor_r += gap_r;
      this.skipped_count_r += gap_r;
    }

    let len = this.cursor_w - this.cursor_r;

    if (8 > len) {
      return false;
    }

    cursor_r = this.cursor_r % this.buf.byteLength;

    let id = this.buf.getUint32(cursor_r, true);
    let count = this.buf.getUint32(cursor_r + 4, true);

    if (count > len - 8) {
      return false;
    }

    if (0 == id) {
      this.skipped_count_r += count;
    } else {
      // console.log("read: " + id + " -> " + count + " bytes");

      this.total_count_r += count;
    }

    this.cursor_r += count + 8;

    return true;
  }

  public write(id: number, buf: ArrayBuffer) {
    let count = buf.byteLength;
    let src = new Uint8Array(buf);

    let cursor_w = this.cursor_w % this.buf.byteLength;
    let gap_w = this.buf.byteLength - cursor_w;

    if (8 > gap_w) {
      this.cursor_w += gap_w;
      this.skipped_count_w += gap_w;
    } else if (count > gap_w) {
      this.buf.setUint32(cursor_w, 0, true);
      this.buf.setUint32(cursor_w + 4, gap_w - 8, true);

      this.cursor_w += gap_w;
      this.skipped_count_w += gap_w - 8;
    }

    cursor_w = this.cursor_w % this.buf.byteLength;

    this.buf.setUint32(cursor_w, id, true);
    this.buf.setUint32(cursor_w + 4, count, true);

    // console.log("wrote: " + id + " -> " + count + " bytes");

    // let dst = new Uint8Array(this.buf.buffer, cursor_w + 8, count);

    for (let i = 0; i < count; ++i) {
      this.buf.setUint8(cursor_w + 8 + i, src[i]);
    }

    this.total_count_w += count;

    // console.log(this.cursor_w);

    this.cursor_w += count + 8;

    // console.log(this.cursor_w);
  }
}

(async () => {
  const urlSearchParams = new URLSearchParams(window.location.search);
  const params = Object.fromEntries(urlSearchParams.entries());

  const wid = void 0 == params.wid ? 96 : parseInt(params.wid);
  const hei = void 0 == params.hei ? 128 : parseInt(params.hei);
  const io_buf_cap = 1024 * 1024;

  document.body.style.margin = "0px";

  const stats = new Stats();
  document.body.appendChild(stats.domElement);
  stats.setMode(0); // 0: fps, 1: ms, 2: mb
  stats.domElement.style.position = "absolute";
  stats.domElement.style.top = "0px"; // align top-left
  stats.domElement.style.left = "0px"; // align top-left
  stats.domElement.style.zIndex = 999999;
  stats.domElement.style.opacity = 1.0;

  const can = document.createElement("canvas");
  document.body.appendChild(can);
  can.style.display = "block";
  can.style.margin = "auto";
  can.style.border = "1px dashed gray";
  can.style.padding = "0px";
  can.style.width = 2 * wid + "px";
  can.style.height = 2 * hei + "px";
  can.style.imageRendering = "pixelated"
  can.width = Math.round(wid * window.devicePixelRatio);
  can.height = Math.round(hei * window.devicePixelRatio);

  const can_ctx2d = can.getContext("2d");
  can_ctx2d.imageSmoothingEnabled = false;

  const { instance } = await WebAssembly.instantiateStreaming(
    fetch("./8bixel.wasm"),
    {
      env: {
        mylog: (value: number) => {
          console.log(value);
        },
      },
    }
  );

  const args_usize_ptr = instance.exports.init(wid, hei, io_buf_cap);
  const args_usize = new DataView(
    instance.exports.memory.buffer,
    args_usize_ptr,
    20
  );

  const canvas_buf_ptr = args_usize.getUint32(0, true);
  const outgo_buf_ptr = args_usize.getUint32(4, true);
  const income_buf_ptr = args_usize.getUint32(8, true);
  const outgo_cursor_w_ptr = args_usize.getUint32(12, true);
  const income_cursor_w_ptr = args_usize.getUint32(16, true);

  const canvas_buf = new Uint8ClampedArray(
    instance.exports.memory.buffer,
    canvas_buf_ptr,
    wid * hei * 4
  );
  const canvas_dat = new ImageData(canvas_buf, wid, hei);

  const outgo_buf = new DataView(
    instance.exports.memory.buffer,
    outgo_buf_ptr,
    io_buf_cap
  );
  const outgo_cursor_w = new DataView(
    instance.exports.memory.buffer,
    outgo_cursor_w_ptr,
    4
  );
  const outgo_writer = new IOHelper(outgo_buf, outgo_cursor_w);

  const income_buf = new DataView(
    instance.exports.memory.buffer,
    income_buf_ptr,
    io_buf_cap
  );
  const income_cursor_w = new DataView(
    instance.exports.memory.buffer,
    income_cursor_w_ptr,
    4
  );
  const income_reader = new IOHelper(income_buf, income_cursor_w);

  if (0 != instance.exports.boot()) {
    return;
  }

  window.addEventListener("keydown", (evt) => {
    new MsgKeyboard(evt.keyCode, 0).write(outgo_writer);
  });
  window.addEventListener("keyup", (evt) => {
    // new MsgKeyboard(evt.keyCode, 1).write(outgo_writer);
  });

  const rAF = () => {
    stats.begin();

    // outgo

    if (0 != instance.exports.ever()) {
      return;
    }

    // income

    can_ctx2d.putImageData(canvas_dat, 0, 0);

    stats.end();

    window.requestAnimationFrame(rAF);
  };
  window.requestAnimationFrame(rAF);
})();
