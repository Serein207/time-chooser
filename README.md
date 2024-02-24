# time-chooser

统计线下验收的最佳时间，在最合适的时间请最多的人同时前来验收，使得总次数最少

## Build

```bash
cargo build --release
```

## Usage

表格中仅支持的时间点如下，如有多个，用中文或英文逗号做分割

- 2.25（周日） 15:00
- 2.25（周日） 19:00
- 2.26（周一） 21:30
- 2.27（周二） 21:30
- 2.28（周三） 15:00
- 2.28（周三） 21:30

表格需要有一行表头，表头的内容可以是任意的，但是F和G列需要分别表示“时间”和“姓名”，时间字段的内容需要符合上面的格式

```bash
Usage: time-chooser --path <PATH> --sheet <SHEET>

Options:
  -p, --path <PATH>    xlsx file path
  -s, --sheet <SHEET>  sheet name
  -h, --help           Print help
  -V, --version        Print version
```

