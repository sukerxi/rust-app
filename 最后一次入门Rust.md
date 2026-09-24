# 最后一次入门Rust

---

## 第 1 周：变量、类型、函数、Struct、Enum 预览

| 天   | 学 6 min            | 写 10 min                                                  | 验收 / commit          |
| ---- | ------------------- | ---------------------------------------------------------- | ---------------------- |
| D1   | The Book §1.1、§1.3 | `cargo new tmdb-organizer`，打印 `Bleach`                  | `cargo run`，`day01`   |
| D2   | §3.1、§3.2          | `let mut episode_count: u32 = 366;` 等变量                 | 打印类型，`day02`      |
| D3   | §3.3                | `fn print_series_info(name: &str, episodes: u32)`          | 调用成功，`day03`      |
| D4   | §3.5 `if`           | `if is_finished { ... } else { ... }`                      | 打印状态，`day04`      |
| D5   | §5.1                | 定义 `struct AnimeFile { name, season, episode }`          | 构造并打印，`day05`    |
| D6   | §5.3                | `impl AnimeFile { fn display_label(&self) -> String }`     | 打印 `S01E01`，`day06` |
| D7   | 复习 §3、§5         | 定义 `enum EpisodeOrder { Tvdb, Dvd, Absolute }` + `match` | 打印不同顺序，`day07`  |

---

## 第 2 周：所有权、借用、切片、String vs &str

| 天   | 学 6 min            | 写 10 min                                                    | 验收 / commit          |
| ---- | ------------------- | ------------------------------------------------------------ | ---------------------- |
| D8   | The Book §4.1       | `let s1 = String::from("Bleach"); let s2 = s1;` 尝试打印 `s1` 看报错 | 理解 move，`day08`     |
| D9   | §4.2 引用           | `fn print_name(name: &str)`，调用 `print_name(&file.name)`   | 不转移所有权，`day09`  |
| D10  | §4.2 可变借用       | `fn add_episode(file: &mut AnimeFile)`，让 episode +1        | 修改成功，`day10`      |
| D11  | §4.3 切片           | `fn first_word(s: &str) -> &str`，从 `Bleach.S01E01.mkv` 返回 `Bleach` | 打印结果，`day11`      |
| D12  | 复习 String vs &str | 写 `fn parse_filename(name: &str) -> Option<(u32, u32)>` 初版 | 解析 `S01E01`，`day12` |
| D13  | 继续 Option         | 让 `parse_filename` 对 `Bleach - 001.mkv` 返回 `None`        | 打印 `Option`，`day13` |
| D14  | 复盘                | 用 `parse_filename` 解析 3 个文件名                          | `cargo run`，`day14`   |

---

## 第 3 周：Vec、String、HashMap、闭包、迭代器

| 天   | 学 6 min      | 写 10 min                                               | 验收 / commit               |
| ---- | ------------- | ------------------------------------------------------- | --------------------------- |
| D15  | The Book §8.1 | `let mut files: Vec<AnimeFile> = vec![];` push 3 个文件 | 打印 `files.len()`，`day15` |
| D16  | §8.2          | 用 `format!` 生成 `S{:02}E{:02}`                        | 生成 label，`day16`         |
| D17  | §8.3          | `HashMap<u32, (u32, u32)>` 存 Absolute→TVDB 映射        | 插入 3 条，`day17`          |
| D18  | §13.1 闭包    | 用闭包按 episode 排序 `Vec<AnimeFile>`                  | 打印排序后，`day18`         |
| D19  | §13.2 迭代器  | `files.iter().filter(|f| f.season == 1).count()`        | 打印数量，`day19`           |
| D20  | §13.3 适配器  | `map` 收集所有 `display_label` 到 `Vec<String>`         | 打印列表，`day20`           |
| D21  | 复盘          | 用迭代器生成一份简单文本报告                            | `cargo run`，`day21`        |

---

## 第 4 周：错误处理 Option、Result、`?`、自定义错误

| 天   | 学 6 min      | 写 10 min                                                    | 验收 / commit           |
| ---- | ------------- | ------------------------------------------------------------ | ----------------------- |
| D22  | The Book §9.1 | 找一个 `unwrap()`，改成 `match` 处理错误                     | 不 panic，`day22`       |
| D23  | §9.2 Result   | `fn parse_episode(s: &str) -> Result<u32, ParseIntError>`    | 打印 Ok/Err，`day23`    |
| D24  | §9.2 `?`      | `parse_filename -> Result<(u32,u32), String>`                | 用 `?` 传播，`day24`    |
| D25  | §9.3 Option   | `fn season_from(name: &str) -> Option<u32>`                  | 打印 None/Some，`day25` |
| D26  | 自定义错误    | `enum ParseError { MissingSeason, MissingEpisode, InvalidNumber }` + `Display` | 能打印错误，`day26`     |
| D27  | 重构          | `parse_filename` 返回 `Result<(u32,u32), ParseError>`        | 编译通过，`day27`       |
| D28  | 复盘          | 给 `parse_filename` 写 3 个 `#[test]` 或手动用例             | `cargo test`，`day28`   |

---

## 第 5 周：枚举与 match 深入，对应截图里的 Order

| 天   | 学 6 min      | 写 10 min                                                    | 验收 / commit         |
| ---- | ------------- | ------------------------------------------------------------ | --------------------- |
| D29  | The Book §6.1 | 定义 `enum EpisodeOrder { Tvdb, Dvd, Absolute, Netflix, Crunchyroll }` | 编译通过，`day29`     |
| D30  | §6.1          | `impl EpisodeOrder { fn label(&self) -> &str }`              | 打印标签，`day30`     |
| D31  | §6.2 match    | `fn parse_order(s: &str) -> Result<EpisodeOrder, String>`    | 解析 `tvdb`，`day31`  |
| D32  | §6.2          | 用 `match` 打印不同 Order 的说明                             | 全部变体覆盖，`day32` |
| D33  | §6.3 `if let` | 定义 `struct Episode { season, episode, title, absolute }`   | 构造一个，`day33`     |
| D34  | 综合          | 定义 `struct RenamePlan { from, to, order }`                 | 构造一个，`day34`     |
| D35  | 复盘          | 手动构造 `RenamePlan` 并打印                                 | `cargo run`，`day35`  |

---

## 第 6 周：泛型、Trait、生命周期基础

| 天   | 学 6 min          | 写 10 min                                                    | 验收 / commit          |
| ---- | ----------------- | ------------------------------------------------------------ | ---------------------- |
| D36  | The Book §10.1    | `fn largest<T: PartialOrd>(list: &[T]) -> &T`                | 用 u32 测试，`day36`   |
| D37  | §10.2 Trait       | `trait EpisodeFormatter { fn format(&self) -> String; }`     | 定义 trait，`day37`    |
| D38  | §10.2             | 为 `AnimeFile` 实现 `EpisodeFormatter`                       | 调用 `format`，`day38` |
| D39  | §10.2 trait bound | `fn print_formatted<T: EpisodeFormatter>(item: &T)`          | 打印，`day39`          |
| D40  | §10.3 生命周期    | `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`          | 测试通过，`day40`      |
| D41  | 抽象数据源        | `trait EpisodeSource { fn episodes(&self, order: &EpisodeOrder) -> Vec<Episode>; }` | fake 实现，`day41`     |
| D42  | 复盘              | 用 trait 抽象输出，替换硬编码打印                            | `cargo run`，`day42`   |

---

## 第 7 周：模块、测试、文档，开始拆文件

| 天   | 学 6 min       | 写 10 min                                       | 验收 / commit         |
| ---- | -------------- | ----------------------------------------------- | --------------------- |
| D43  | The Book §7.1  | 新建 `src/models.rs`，放 `AnimeFile`、`Episode` | 编译通过，`day43`     |
| D44  | §7.2           | 新建 `src/parser.rs`，放 `parse_filename`       | 编译通过，`day44`     |
| D45  | §7.3           | `main.rs` 写 `mod models; mod parser; use ...`  | 调用成功，`day45`     |
| D46  | §11.1 单元测试 | 在 `parser.rs` 写 `#[cfg(test)] mod tests`      | `cargo test`，`day46` |
| D47  | §11.2 集成测试 | 新建 `tests/parser_test.rs`                     | 测试公共 API，`day47` |
| D48  | §11.3          | 补 3 个边界测试：缺季、缺集、乱码               | 全绿，`day48`         |
| D49  | 文档           | 给公共函数写 `///`，`cargo fmt`                 | `cargo test`，`day49` |

---

## 第 8 周：文件系统，扫描本地目录

| 天   | 学 6 min               | 写 10 min                                                    | 验收 / commit             |
| ---- | ---------------------- | ------------------------------------------------------------ | ------------------------- |
| D50  | `std::path::Path` 文档 | `fn scan_dir(dir: &Path) -> Result<Vec<PathBuf>, io::Error>` | 返回路径列表，`day50`     |
| D51  | `std::fs::read_dir`    | 遍历目录，打印文件名                                         | `cargo run -- .`，`day51` |
| D52  | `Path::extension`      | 只保留 `.mkv`、`.mp4`                                        | 过滤成功，`day52`         |
| D53  | `metadata()`           | 打印文件大小                                                 | 输出 bytes，`day53`       |
| D54  | 综合                   | 把路径转成 `AnimeFile`，解析失败就跳过                       | 生成列表，`day54`         |
| D55  | `tempfile`             | `cargo add tempfile --dev`，写扫描测试                       | `cargo test`，`day55`     |
| D56  | 复盘                   | 扫描你本地真实《死神》目录                                   | 打印结果，`day56`         |

---

## 第 9 周：CLI 参数 clap

| 天   | 学 6 min  | 写 10 min                                                    | 验收 / commit                  |
| ---- | --------- | ------------------------------------------------------------ | ------------------------------ |
| D57  | clap 文档 | `cargo add clap --features derive`，定义 `Args { dir: PathBuf }` | 编译通过，`day57`              |
| D58  | clap 文档 | 用 `Args` 替换硬编码路径                                     | `cargo run -- .`，`day58`      |
| D59  | clap 文档 | 加 `--order`，默认 `tvdb`，解析成 `EpisodeOrder`             | 参数生效，`day59`              |
| D60  | clap 文档 | 加 `--dry-run` 布尔参数                                      | 打印 dry-run，`day60`          |
| D61  | clap 文档 | 加 `--verbose` 或 `--top`                                    | 参数生效，`day61`              |
| D62  | README    | 写安装、用法、示例                                           | `--help` 正确，`day62`         |
| D63  | 复盘      | 所有 CLI 参数跑一遍                                          | `cargo run -- --help`，`day63` |

---

## 第 10 周：HTTP 请求 TMDB，dotenv + ureq

| 天   | 学 6 min     | 写 10 min                                                    | 验收 / commit          |
| ---- | ------------ | ------------------------------------------------------------ | ---------------------- |
| D64  | TMDB 注册    | 创建 `.env` 写 `TMDB_API_KEY=...`，`.gitignore` 加 `.env`    | 不泄露 key，`day64`    |
| D65  | dotenvy 文档 | `cargo add dotenvy ureq`，读取 API key                       | 打印 key 长度，`day65` |
| D66  | ureq 文档    | GET `/search/tv?query=Bleach&api_key=...`                    | 拿到响应，`day66`      |
| D67  | ureq 文档    | 打印响应前 500 字符                                          | 看到 JSON，`day67`     |
| D68  | HTTP 状态    | status 非 200 返回 Err                                       | 错误处理，`day68`      |
| D69  | 封装         | `fn search_series(name: &str) -> Result<String, Box<dyn Error>>` | 编译通过，`day69`      |
| D70  | 复盘         | 搜索《死神》，打印 ID                                        | `cargo run`，`day70`   |

---

## 第 11 周：Serde JSON 反序列化

| 天   | 学 6 min    | 写 10 min                                                    | 验收 / commit      |
| ---- | ----------- | ------------------------------------------------------------ | ------------------ |
| D71  | serde 文档  | `cargo add serde --features derive serde_json`，定义 `SearchResponse { results: Vec<SearchItem> }` | 编译通过，`day71`  |
| D72  | serde 文档  | 定义 `SearchItem { id, name, first_air_date }`               | 编译通过，`day72`  |
| D73  | serde_json  | `serde_json::from_str::<SearchResponse>`                     | 解析成功，`day73`  |
| D74  | Option 字段 | `first_air_date: Option<String>`                             | 处理 null，`day74` |
| D75  | 综合        | 从 results 里取第一个 ID                                     | 打印 ID，`day75`   |
| D76  | 封装        | `fn get_series_id(name: &str) -> Result<u32, Box<dyn Error>>` | 调用成功，`day76`  |
| D77  | 复盘        | 找不到剧集时返回 Err                                         | 错误提示，`day77`  |

---

## 第 12 周：TMDB Episode Groups API，拿到不同 Order

| 天   | 学 6 min  | 写 10 min                                                    | 验收 / commit               |
| ---- | --------- | ------------------------------------------------------------ | --------------------------- |
| D78  | TMDB 文档 | 请求 `/tv/{series_id}/episode_groups`，打印 JSON             | 看到 groups，`day78`        |
| D79  | serde     | 定义 `EpisodeGroupResponse { results: Vec<EpisodeGroup> }`   | 解析成功，`day79`           |
| D80  | 筛选      | 定义 `EpisodeGroup { id, name, group_count }`，列出所有组    | 找到 TVDB/Absolute，`day80` |
| D81  | TMDB 文档 | 请求 `/tv/{id}/episode_group/{group_id}`                     | 拿到详情，`day81`           |
| D82  | serde     | 定义 `GroupDetail { groups: Vec<Group> }`、`Group { episodes }`、`GroupEpisode { season_number, episode_number, name }` | 解析成功，`day82`           |
| D83  | 转换      | 把 `GroupEpisode` 转成你的 `Episode`                         | 生成 Vec，`day83`           |
| D84  | 复盘      | 打印 TVDB Order 前 5 集                                      | `cargo run`，`day84`        |

---

## 第 13 周：核心映射，生成重命名计划

| 天   | 学 6 min | 写 10 min                                                    | 验收 / commit         |
| ---- | -------- | ------------------------------------------------------------ | --------------------- |
| D85  | 设计函数 | `fn build_plan(local: &[AnimeFile], remote: &[Episode], order: EpisodeOrder) -> Vec<RenamePlan>` | 签名编译，`day85`     |
| D86  | 匹配     | 按 `season + episode` 匹配本地和远程                         | 生成计划，`day86`     |
| D87  | Absolute | 用 `HashMap<u32,(u32,u32)>` 做 Absolute→TVDB 映射            | 映射成功，`day87`     |
| D88  | 缺失处理 | 加 `PlanStatus::Missing`                                     | 不 panic，`day88`     |
| D89  | 新文件名 | 生成 `Bleach - S01E01 - 标题.mkv`，清理非法字符              | 打印新名，`day89`     |
| D90  | 计划表   | 打印计划表格，不操作文件                                     | 终端可读，`day90`     |
| D91  | 复盘     | 用假数据测试计划生成                                         | `cargo test`，`day91` |

---

## 第 14 周：文件操作、dry-run、安全确认

| 天   | 学 6 min         | 写 10 min                               | 验收 / commit         |
| ---- | ---------------- | --------------------------------------- | --------------------- |
| D92  | `create_dir_all` | 创建目标季文件夹                        | 目录出现，`day92`     |
| D93  | `fs::rename`     | 重命名单个文件                          | 文件改名，`day93`     |
| D94  | dry-run          | `--dry-run` 只打印不执行                | 不操作文件，`day94`   |
| D95  | 安全确认         | 无 `--yes` 时要求输入 `y/n`             | 可取消，`day95`       |
| D96  | 冲突处理         | 目标已存在则跳过或加后缀                | 不覆盖，`day96`       |
| D97  | 集成测试         | 用 tempdir 造文件，运行计划，断言重命名 | `cargo test`，`day97` |
| D98  | 复盘             | 在复制的小目录实测                      | 成功整理，`day98`     |

---

## 第 15 周：完善、模块化、README、发布准备

| 天   | 学 6 min | 写 10 min                                        | 验收 / commit      |
| ---- | -------- | ------------------------------------------------ | ------------------ |
| D99  | anyhow   | `cargo add anyhow`，`main -> anyhow::Result<()>` | 错误简化，`day99`  |
| D100 | 日志     | 加 `--verbose`，用 `eprintln!` 或 tracing        | 调试信息，`day100` |
| D101 | 模块化   | 拆 `cli.rs`、`tmdb.rs`、`plan.rs`、`rename.rs`   | 编译通过，`day101` |
| D102 | README   | 安装、配置 API key、用法、示例                   | 别人能跑，`day102` |
| D103 | clippy   | `cargo fmt && cargo clippy -- -D warnings`       | 无警告，`day103`   |
| D104 | 测试     | `cargo test` 全绿，补边界测试                    | 全绿，`day104`     |
| D105 | release  | `cargo build --release`，真实目录实测            | 可用，`day105`     |

---

## 第 16 周：毕业、GitHub、Release、写“不再入门”

| 天   | 学 6 min | 写 10 min                                                    | 验收 / commit                  |
| ---- | -------- | ------------------------------------------------------------ | ------------------------------ |
| D106 | 实测     | 用工具整理复制版《死神》，记录问题                           | 问题清单，`day106`             |
| D107 | 修复     | 修最烦的问题 1                                               | commit，`day107`               |
| D108 | 修复     | 修问题 2                                                     | commit，`day108`               |
| D109 | CI       | GitHub Actions：test、clippy、fmt                            | push 自动跑，`day109`          |
| D110 | 元数据   | 加 LICENSE，补 `Cargo.toml`                                  | `cargo package` 通过，`day110` |
| D111 | 发布     | 发 GitHub Release `v1.0.0`                                   | Release 页面，`day111`         |
| D112 | 毕业     | README 顶部写：“我不再入门 Rust。我用 Rust 写了 tmdb-organizer。” | 复盘 16 周，`day112`           |

---

## 依赖添加时间线

- 第 8 周：`cargo add tempfile --dev`
- 第 9 周：`cargo add clap --features derive`
- 第 10 周：`cargo add dotenvy ureq`
- 第 11 周：`cargo add serde --features derive serde_json`
- 第 15 周：`cargo add anyhow`

## 最终文件结构

```text
tmdb-organizer/
├── src/
│   ├── main.rs
│   ├── cli.rs
│   ├── models.rs
│   ├── parser.rs
│   ├── tmdb.rs
│   ├── plan.rs
│   └── rename.rs
├── tests/
│   └── parser_test.rs
├── .env
├── .gitignore
├── Cargo.toml
└── README.md
```

## 最后三句

1. 每天 20 分钟到点就停，状态好也不多写，第二天休息。
2. 卡住 3 分钟就写 `// TODO` 跳过，别在一行报错上耗完 20 分钟。
3. 第 112 天不是“学完 Rust”，而是“我用 Rust 做出了一个能整理《死神》剧集的工具”。
