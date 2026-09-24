# 最后一次入门Rust

> **怎么用这份 README**
> 每天 20 分钟：`学 6 min` 点开当天链接把那节读完 → `写 10 min` 照着敲一遍 → `验收` 里跑通就 commit。
>
> **教程源**（大陆均可直接访问）
> - [《Rust 程序设计语言》简体中文版](https://kaisery.github.io/trpl-zh-cn/) —— 主教程，表格里记作「书」
> - [中文标准库文档](https://rustwiki.org/zh-CN/std/) —— 表格里记作「std」
> - [docs.rs](https://docs.rs/) —— 第三方 crate 文档，表格里记作「docs」
>
> **正文**：每周表格下面是当天的讲解 + 示例代码 + 易错点，可以完全脱离外部链接自学。

## 目录

- [第 1 周：变量、类型、函数、Struct、Enum 预览](#w1)
- [第 2 周：所有权、借用、切片、String vs &str](#w2)
- [第 3 周：Vec、String、HashMap、闭包、迭代器](#w3)
- [第 4 周：错误处理 Option、Result、`?`、自定义错误](#w4)
- [第 5 周：枚举与 match 深入](#w5)
- [第 6 周：泛型、Trait、生命周期基础](#w6)
- [第 7 周：模块、测试、文档](#w7)
- [第 8 周：文件系统，扫描本地目录](#w8)
- [第 9 周：CLI 参数 clap](#w9)
- [第 10 周：HTTP 请求 TMDB，dotenv + ureq](#w10)
- [第 11 周：Serde JSON 反序列化](#w11)
- [第 12 周：TMDB Episode Groups API](#w12)
- [第 13 周：核心映射，生成重命名计划](#w13)
- [第 14 周：文件操作、dry-run、安全确认](#w14)
- [第 15 周：完善、模块化、README、发布准备](#w15)
- [第 16 周：毕业、GitHub、Release](#w16)

---

<a id="w1"></a>

## 第 1 周：变量、类型、函数、Struct、Enum 预览

| 天   | 学 6 min                                      | 写 10 min                                                  | 验收 / commit          |
| ---- | --------------------------------------------- | ---------------------------------------------------------- | ---------------------- |
| D1   | cargo 项目结构与 `println!` 宏（[书 §1.3](https://kaisery.github.io/trpl-zh-cn/ch01-03-hello-cargo.html)） | `cargo new tmdb-organizer`，打印 `Bleach`                  | `cargo run`，`day01`   |
| D2   | 变量、`mut` 与标量类型（[书 §3.1](https://kaisery.github.io/trpl-zh-cn/ch03-01-variables-and-mutability.html)、[§3.2](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html)） | `let mut episode_count: u32 = 366;` 等变量                 | 打印类型，`day02`      |
| D3   | 函数、参数类型与返回值（[书 §3.3](https://kaisery.github.io/trpl-zh-cn/ch03-03-how-functions-work.html)） | `fn print_series_info(name: &str, episodes: u32)`          | 调用成功，`day03`      |
| D4   | `if` / `else` 条件表达式（[书 §3.5](https://kaisery.github.io/trpl-zh-cn/ch03-05-control-flow.html)） | `if is_finished { ... } else { ... }`                      | 打印状态，`day04`      |
| D5   | `struct` 定义与实例化（[书 §5.1](https://kaisery.github.io/trpl-zh-cn/ch05-01-defining-structs.html)） | 定义 `struct AnimeFile { name, season, episode }`          | 构造并打印，`day05`    |
| D6   | `impl` 方法与 `&self`（[书 §5.3](https://kaisery.github.io/trpl-zh-cn/ch05-03-method-syntax.html)） | `impl AnimeFile { fn display_label(&self) -> String }`     | 打印 `S01E01`，`day06` |
| D7   | `enum` 与 `match`（[书 §6.1](https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html)、[§6.2](https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html)） | 定义 `enum EpisodeOrder { Tvdb, Dvd, Absolute }` + `match` | 打印不同顺序，`day07`  |

### D1 · cargo 项目结构与 `println!` 宏

📖 [Hello, Cargo!](https://kaisery.github.io/trpl-zh-cn/ch01-03-hello-cargo.html)

**为什么要有 cargo。** 一个项目最终会牵扯四件事：依赖从哪来、按什么顺序编译、产物放哪、测试怎么跑。cargo 把它们压成一套约定：`Cargo.toml` 是**清单**（包名、版本、依赖），`src/` 放源码，`target/` 统一收产物与缓存，命令统一成 `cargo run / build / test / doc`。约定大于配置的好处是——你打开任何 Rust 仓库都不用先找入口在哪。注意 `target/` 已被写进 `.gitignore`：它只是可重建的中间产物，删掉重编即可，不该进版本库。

**为什么 `println!` 带感叹号。** 带 `!` 的是**宏**，在**编译期**被展开成代码。它解决了一件普通函数解决不了的事：参数个数不定、类型不定，但 `{}` 的个数必须和参数个数在**编译期**就对得上。普通函数的参数列表在定义时就固定了，做不到。所以 `!` 不是语法糖，少写会直接报 `cannot find function println in this scope`。`{}` 是 `Display` 占位符，`{:?}` 是 `Debug` 占位符（D5 会用），`{name}` 是**捕获式内联**，等价于 `{}` 后面跟 `name`。

```rust
fn main() {
    println!("Bleach!");          // 打印字面量
    println!("{} 集", 366);        // {} 按顺序取后面的参数
    let name = "Bleach";
    println!("{name} 已完结");      // 捕获式内联，等价于 println!("{} 已完结", name)
    println!("{:?}", (1, "S01E01")); // {:?} 打印调试形态，能一次看整个值的结构
}
```

⚠️ 两个高频坑：漏掉 `!`；`{}` 个数与参数个数对不上（多了报 `argument never used`，少了报 `invalid reference to argument`）。暂时不会 `{}`/`{:?}` 该用哪个时，先用 `{:?}`——`Debug` 的适用范围比 `Display` 广得多。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `cargo run` 每次都要全量重编 | 增量编译：只重编改动的 crate，依赖走缓存。第一次之后通常秒级 |
| `target/` 是一部分源码，要提交 | 纯产物，删了 `cargo run` 会重建；它已在 `.gitignore` 里，提交只会撑爆仓库 |
| `println!` 是个函数 | 是宏，编译期展开，所以能收可变参数、还能在编译期校验占位符数量 |

**在 tmdb-organizer 里**：`src/main.rs` 的 `main` 就是整个整理工具的入口，第 1-8 周所有练习都直接写在这个文件里；第 9 周引入 CLI 之后，`main` 才会收敛成"解析参数 → 调库函数"两件事。

### D2 · 变量、`mut` 与标量类型

📖 [变量与可变性](https://kaisery.github.io/trpl-zh-cn/ch03-01-variables-and-mutability.html)、[数据类型](https://kaisery.github.io/trpl-zh-cn/ch03-02-data-types.html)

**为什么默认不可变。** `let` 出来的变量默认改不了，想改必须显式加 `mut`。这不是限制，而是为了让**读代码的人**不必在脑子里跟踪"这个值后面会不会被改"——看到 `let` 就能确定"从这行到作用域结束它一直是这个值"。能排除的路径越多，排 bug 越快；编译器也能据此优化。写 `mut` 是对编译器和同事同时声明"我确认它会变"。

**`mut` 和遮蔽（shadowing）不是一回事。** 还有一种"看起来在改"的写法：再写一个同名的 `let`。它其实是**造了个新变量**，旧的只是被遮住。关键区别是——遮蔽**可以换类型**，`mut` 不行。

```rust
fn main() {
    // 遮蔽：新变量，可以换类型
    let episodes = "366";                            // &str
    let episodes: u32 = episodes.parse().unwrap();    // 换成 u32，同名变量被遮住

    // mut：同一个变量，类型不能变
    let mut count: u32 = 0;
    count += 1;

    let series = "Bleach";      // 不可变
    let ratio = 0.75f64;        // f64
    let is_finished = false;    // bool
    let initial = 'B';          // char：单引号，一个 Unicode 字符（4 字节）

    println!("{series} {count} {episodes} {ratio} {is_finished} {initial}");
}
```

**整数类型怎么选。** 尺寸决定取值范围：`u32` 无符号（0 ~ 42 亿），`i32` 有符号（正负各半），`usize` 是指针宽度（64 位机器上是 64 位，集合的长度和下标都是它）。整数字面量默认推断成 `i32`，浮点默认 `f64`。实用原则：**集数、季数、TMDB ID 用 `u32`**（不可能是负数，范围也够），**下标、长度用 `usize`**（和 `Vec::len()` 类型一致，省得来回 `as`）。

**溢出行为值得先知道。** 超出类型范围时，debug 下（`cargo run`）直接 panic 报 `attempt to add with overflow`，release 下（`cargo run --release`）默认**回绕**（`u32::MAX + 1` 变 0）且不报错。想明确表达意图就用 `wrapping_add`（回绕）、`saturating_add`（顶到上限）、`checked_add`（返回 `Option`）。后面处理真实集数时会用到。

⚠️ 忘写 `mut` 会在赋值处报 `cannot assign twice to immutable variable`，错误信息会主动建议加 `mut`。能推断出来的地方**别写类型标注**（`let mut count = 0;` 更常见）；必须写的典型是 `parse()`——它不知道该转成 `u32` 还是 `i64`，只能靠左边的标注或 `let n: u32 = s.parse().unwrap();`。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `let` 就是别的语言的 `const` | `let` 是**运行期**变量；Rust 的 `const` 才是编译期常量，必须写类型、可放在函数外 |
| 遮蔽会立刻释放旧变量 | 旧值仍在，只是名字不可达；它要等**作用域结束**才被释放 |
| `u32` 和 `usize` 能混着算 | 不会自动转换，`u32 + usize` 报 `mismatched types`，得显式 `as usize` |
| 溢出总会 panic | 只有 debug 下才 panic；release 默认静默回绕，这类 bug 只在发布版出现 |

**在 tmdb-organizer 里**：`season` / `episode` 用 `u32`（对齐 TMDB API 返回的 `season_number`、`episode_number`），而遍历 `Vec` 拿到的下标是 `usize`——第 13 周做集数映射时必然要在两者之间 `as usize` 转换，提前有心理准备。

### D3 · 函数、参数类型与返回值

📖 [函数](https://kaisery.github.io/trpl-zh-cn/ch03-03-how-functions-work.html)

**为什么最后一行不加分号就等于返回。** Rust 是**表达式语言**：一个块 `{ ... }` 的值，就是它最后一行**表达式**的值；加上分号它就从"表达式"变成"语句"，值是 `()`（空元组）。所以函数的返回不是"某个 return 关键字的工作"，而是"函数体的值"。`return` 只在**提前返回**时才需要写，正常结尾写 `return x;` 反而风格不佳。理解了这一点，D4 的 `if` 能赋值、D7 的 `match` 能赋值就都顺理成章了。

**为什么参数必须写类型、还不做类型推断。** 函数是模块之间的**边界**：别人只看签名就要知道你收什么、给什么。如果参数类型靠推断，那推断范围会顺着调用链扩散，编译变慢、报错信息也会变成一堆不知所云的约束。所以 Rust 的选择是：**函数签名显式标注，函数体内才是推断的**。返回类型同样必须写。

**为什么参数用 `&str` 而不是 `String`。** `String` 能自动 `Deref` 成 `&str`，所以形参写 `&str` 时，`"Bleach"` 字面量和 `&my_string` 都能直接传，调用方零成本。反过来形参写 `String`，调用方就必须 `clone()` 或者把所有权交出去——这是新手最常见的性能与借用问题来源。一句话原则：**参数优先借用，需要拥有时才收 `String`**。

```rust
fn print_series_info(name: &str, episodes: u32) {
    println!("{name} 共 {episodes} 集");     // 无返回值，末尾是语句
}

fn episode_label(season: u32, episode: u32) -> String {
    format!("S{:02}E{:02}", season, episode)  // 无分号 → 这就是返回值
}

fn label_or_unknown(season: u32) -> String {
    if season == 0 {
        return String::from("S??");          // 提前返回才用 return
    }
    format!("S{:02}", season)
}

fn main() {
    print_series_info("Bleach", 366);
    println!("{}", episode_label(1, 1));      // S01E01
    println!("{}", label_or_unknown(0));      // S??
}
```

⚠️ 最常见的报错是在最后一行手滑加了 `;`：函数返回 `()`，而签名声明返回 `String`，报 `mismatched types: expected String, found ()`——注意错误会指向签名而不是最后一行。另外 `format!` 和 `println!` 的区别要记牢：前者**返回** `String`，后者**打印**并返回 `()`。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 所有函数都要用 `return` 结尾 | 只在提前返回时写；正常结尾用表达式。多余 `return` 会被 clippy 提示 `needless_return` |
| 函数可以不写返回类型 | 必须写。只有返回 `()` 时可以省略 `-> ()` |
| 参数写 `&str` 是为了少复制，可能有语法开销 | 传的是"指针 + 长度"两个机器字（胖指针），不碰堆数据，零成本 |
| 函数可以在定义之后调用所以顺序无所谓 | 顺序确实无所谓（不像 C 需要前置声明），但**类型必须写全** |

**在 tmdb-organizer 里**：整个工具最后会长成一条函数链——`scan_dir(&Path) -> Vec<PathBuf>` → `parse_filename(&str) -> Option<(u32, u32)>`（D12）→ `build_plan(...) -> Vec<RenamePlan>` → `exec_plan(&[RenamePlan])`。它们清一色**借用输入、返回新数据**，正是 D3 这两条原则的直接应用。

### D4 · `if` / `else` 条件表达式

📖 [控制流](https://kaisery.github.io/trpl-zh-cn/ch03-05-control-flow.html)

**为什么 Rust 没有三元运算符。** 因为 `if` 本身**就是表达式**，能直接当值用，`let status = if c { a } else { b };` 就顶替了别的语言的 `c ? a : b`。代价是两个分支必须返回**同一种类型**——编译器要为整个 `if` 表达式定出一个确定的类型，所以 `if c { 1 } else { 1.0 }` 会报错，Rust 不做隐式数值提升。

**为什么条件必须是 `bool`。** 别的语言里 `if (1)`、`if (ptr)`、`if ("text")` 都当"真"处理，代价是 `if (x = 5)` 这种把赋值写成比较的 bug 完全合法，调试半天。Rust 要求条件严格是 `bool`：`if 1 {` 报 `expected bool, found integer`；而 `if x = 5 {` 也过不了，因为赋值的值是 `()` 而不是 `bool`。这条"不做隐式真值转换"的规则，用一个编译错误换掉了一整类难查的 bug。

**表达式还能用在别处。** 因为块的值就是最后一行的值，你可以把 `if` 塞进任何要值的位置——函数末尾、`let` 右边、甚至 `match` 分支里；反过来，如果 `if` 用在语句位置（末尾没当成值用），它整体就是 `()`，此时两个分支返回不同类型也无所谓（只要别用它的值）。

```rust
fn main() {
    let is_finished = false;

    if is_finished {
        println!("已完结，可以整理文件了");
    } else {
        println!("连载中，先别动");
    }

    // if 是表达式：直接赋值
    let status = if is_finished { "完结" } else { "连载" };
    println!("状态：{status}");

    // 串联判断，从窄到宽
    let episodes = 0;
    let kind = if episodes == 0 {
        "未知"
    } else if episodes > 100 {
        "长篇"
    } else {
        "常规"
    };
    println!("{kind}");
}
```

⚠️ 除了"条件必须 `bool`"，第二坑是分支类型不一致：`let x = if c { "a" } else { 1 };` 报 `if and else have incompatible types`，错误信息里会贴出两个分支的具体类型。第三坑是把 `if` 当值的用法忘了加 `else`——那样可能没有值可取，编译器直接报 `expected else`。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `if` 后面不写括号会出错 | Rust 的 `if` 条件**不加括号**，写成 `if (c)` 只会得到 clippy 的 `unused_parens` 提示 |
| 分支值不同可以让编译器自动转换 | 不会。`1` 和 `1.0`、`&str` 和 `String` 都算类型不同，直接报错 |
| 用在语句位置时两个分支类型也得一样 | 不必。此时整个 `if` 的值是 `()`，两个分支各返回什么都没关系 |
| `if` 和 `match` 的取舍是风格问题 | 不是。判断一个 `bool` 用 `if`；判断**是哪种情况**用 `match`，才能拿到穷尽性检查 |

**在 tmdb-organizer 里**：第 14 周的 `--dry-run` 就是这一行的样子——`if dry_run { 打印重命名计划 } else { 真正改文件名 }`；而"这个文件该用哪种顺序解析"会走 D7 的 `match`，不是 `if`。

### D5 · `struct` 定义与实例化

📖 [结构体的定义和实例化](https://kaisery.github.io/trpl-zh-cn/ch05-01-defining-structs.html)

**为什么用 struct 而不是一堆变量或元组。** `struct` 就是把几个相关的值捆成一个**命名**的整体。捆起来的意义有两层：一层是**可读性**——`file.episode` 比 `file.2` 或 `episode` 强太多，字段名本身就是文档；另一层是**类型安全**——`AnimeFile` 和 `struct MovieFile { ... }` 即使字段一模一样也是两个不同类型，编译器不会让你传错。这就是所谓的"用类型表达意图"。

**从所有权角度看 struct。** 上面 `name: String` 是**拥有**堆上数据的字段。当一个 `AnimeFile` 被创建时，`String` 的所有权就移进这个 struct 里了；struct 离开作用域被销毁时，它的字段会**一起被释放**——这是 Rust 不需要 GC 也不需要手写 `free` 的关键机制（D8 会正式展开）。顺带一个直接推论：既然 `AnimeFile` 拥有 `name`，那么 `let name = String::from("Bleach"); AnimeFile { name, .. }` 之后，原来的 `name` 就不能再用了。

**`#[derive(Debug)]` 是什么。** 它是**派生宏**，在编译期帮你生成 `impl Debug for AnimeFile { ... }` 这段代码。没它的话 `println!("{file:?}")` 会报 `AnimeFile doesn't implement Debug`。同理 `#[derive(Clone, PartialEq)]` 分别生成"能 `.clone()`"和"能用 `==` 比较"的实现。这是 Rust 里"零成本抽象"的典型例子：写一行属性，展开成和你手写一样的代码，运行时没有任何额外开销。

```rust
#[derive(Debug, Clone)]        // 生成 Debug（可用 {:?}）和 Clone（可 .clone()）
struct AnimeFile {
    name: String,
    season: u32,
    episode: u32,
}

fn main() {
    let name = String::from("Bleach");
    let season = 1;

    // 字段初始化简写：变量名和字段名一致时可以只写一次
    let file = AnimeFile { name, season, episode: 1 };

    println!("{file:?}");                       // AnimeFile { name: "Bleach", season: 1, episode: 1 }
    println!("{file:#?}");                      // {:#?} 是多行美化输出，字段多时更好看
    println!("{} S{}", file.name, file.season); // 也可以逐字段取用

    let same = file.clone();                    // 需要 Clone；不加 derive 这里会报错
    println!("{}", same.name);
}
```

⚠️ 三个坑：一是 `println!("{file}")` 报 `doesn't implement std::fmt::Display`——默认只有 `Debug`，得写 `{file:?}`；二是简写 `AnimeFile { name, season }` 要求变量名和字段名**逐字相同**，少一个字段也会报 `missing field episode in initializer`；三是 struct 里字段的**顺序不影响正确性**，但改了字段顺序会让内存布局变化（结构体字段多时有对齐填充）。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `#[derive(Debug)]` 是运行期反射 | 编译期生成的普通 `impl`，运行时零开销；`cargo expand` 能看到展开后的代码 |
| 字段相同的两个 struct 可以互相赋值 | 不行，是两个完全不同的类型，报 `expected AnimeFile, found MovieFile` |
| `{file:?}` 和 `{file:#?}` 只是写法不同 | `{:#?}` 是**多行美化**输出，调试嵌套结构时明显更好读 |
| struct 的字段可以像 JS 对象那样随时增删 | 字段集合在编译期固定，这也是它比 `HashMap` 快得多的原因 |

**在 tmdb-organizer 里**：`AnimeFile` 是第 8 周扫描目录后要装进 `Vec` 的核心类型（名字、季、集，后续还会加 `path: PathBuf`）。一开始就 `#[derive(Debug, Clone)]`——`Debug` 给日志和 dry-run 打印用，`Clone` 给"复制一份计划、原数据不动"用。

### D6 · `impl` 方法与 `&self`

📖 [方法语法](https://kaisery.github.io/trpl-zh-cn/ch05-03-method-syntax.html)

**方法 vs 关联函数，区别只在第一个参数。** `impl` 块把函数挂到类型上。第一个参数是 `self`（或它的引用）的叫**方法**，用 `f.method()` 调，编译器会自动把 `f` 塞进第一个位置；第一个参数不是 `self` 的叫**关联函数**，用 `Type::func()` 调，`AnimeFile::new(...)` 就是标准例子（`new` 只是社区约定，不是语言关键字）。这两种在编译后其实没有本质区别，只是**调用语法**上的糖。

**`self` 的三种形式，就是 D2-D3 所有权规则的复用。** 记住这张对应表，D8 学所有权时会轻松很多：

| 写法 | 含义 | 调用后还能用 `f` 吗 |
| ---- | ---- | ------------------- |
| `self` | 拿走所有权（move） | 不能，已被消费 |
| `&self` | 只读借用 | 能，只读 |
| `&mut self` | 可变借用，可改字段 | 能，且改动会保留 |

所以 `fn display_label(&self)` 选 `&self`，是因为"打印标签"这个动作不需要销毁数据；它如果写成 `self`，`f.display_label()` 之后 `f` 就失效了。选择哪个不是风格问题，而是**这个操作到底需不需要拿到所有权**。

**为什么返回 `String` 而不是 `&str`。** `format!` 每次都会新造一段堆数据，这段数据必须有个主人。返回 `&str` 就得指回某个已经存在的字符串，而这里并不存在，所以只能返回拥有所有权的 `String`——所有权从函数内部移交给调用方，函数结束时不会把它释放掉。记住一个判定法：**返回借用（`&str`）意味着"我指给你看别人的数据"，返回 `String` 意味着"我造了一份新的给你"**。

```rust
#[derive(Debug)]
struct AnimeFile { name: String, season: u32, episode: u32 }

impl AnimeFile {
    fn new(name: &str, season: u32, episode: u32) -> Self {   // 关联函数
        Self { name: name.to_string(), season, episode }      // Self 就是 AnimeFile
    }

    fn display_label(&self) -> String {                       // 只读方法
        format!("S{:02}E{:02}", self.season, self.episode)
    }

    fn bump_episode(&mut self) {                              // 可变方法
        self.episode += 1;
    }

    fn into_name(self) -> String {                            // 消费方法：self 会被吃掉
        self.name
    }
}

fn main() {
    let mut f = AnimeFile::new("Bleach", 1, 1);
    println!("{}", f.display_label());  // S01E01
    f.bump_episode();
    println!("{}", f.display_label());  // S01E02
    let name = f.into_name();           // f 在这里被消费
    println!("{name}");
    // println!("{f:?}");               // 取消注释会报 use of moved value: `f`
}
```

⚠️ 两个高频坑：第一，把只读方法写成 `fn display_label(self)` 后，调用处接着用 `f` 就报 `use of moved value`——错误会指向**调用行**，但根因在定义处；第二，`fn name(&self) -> String { self.name }` 会报 `cannot move out of self.name which is behind a shared reference`，因为 `self` 只是借用、你还想搬走它的字段。这里选 `self.name.clone()`（复制一份）或改成 `fn name(&self) -> &str { &self.name }`（借出去，零拷贝，一般更优）。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 方法必须和 struct 写在同一个文件 | 不必，另一个文件里写 `impl AnimeFile { ... }` 完全合法（第 7 周拆模块时就这么干） |
| `&self` 方法里改字段只是被忽略 | 直接编译错误：`cannot assign to self.episode, which is behind a & reference`。要改必须 `&mut self` |
| 用 `&self` 就保证不复制数据 | `&self` 保证的是**调用**不复制；方法体里写 `.clone()` 照样复制 |
| 调用者变量不用 `mut` 也能调 `&mut self` 方法 | 不行，报 `cannot borrow f as mutable`。变量本身必须是 `let mut f` |

**在 tmdb-organizer 里**：后面会这么用这套规则——`AnimeFile::parse(&str) -> Option<Self>` 当构造器（关联函数，收 `&str` 因为它只借用文件名）；`display_label(&self) -> String` 当打印方法（只读，因为 dry-run 打印时不能把数据消费掉）；而真正要改文件名时用的是 `fs::rename(&plan.from, &plan.to)`，压根不需要 `&mut self`——**只有在确实要改动数据时，才让方法拿可变所有权**。

### D7 · `enum` 与 `match`

📖 [枚举的定义](https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html)、[`match` 控制流结构](https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html)

**枚举 = "多选一"，而且每个选项可以自带数据。** 别的语言里 enum 往往只是给整数起名字（`const TVDB = 0`），Rust 的枚举是**带标签的联合体**（tagged union）：值在任意时刻只属于**一个**变体，每个变体可以携带不同类型的数据。内存布局大致是「一个判别值 + 按最大变体算的空间」，所以它不比自己手写一个 `(tag, payload)` 组合更贵，但编译器保证了**你不可能同时处于两个变体**——这正是它安全的原因。

**这也是为什么 Rust 没有 `null`。** 想表达"可能有值、可能没有"，就定义一个枚举：`enum Option<T> { Some(T), None }`。编译器强制你在使用前先 `match` 出到底是哪种情况，于是"忘记判空"从一个运行时崩溃变成一个**编译错误**。`Result<T, E>` 同理（第 4 周）。可以说 Rust 把"空值"和"错误"这两种普遍存在的状态，从约定变成了类型系统里的东西。

**`match` 的核心价值是穷尽性检查。** `match` 是表达式（和 D4 的 `if` 一样），所有分支必须返回同类型。它的编译期保证是：**必须覆盖所有变体**，漏一个就报错，并且错误信息会点名漏掉哪个。更有价值的是它的"反向价值"——当你在 `EpisodeOrder` 里加一个新变体 `Absolute` 后，所有没处理它的 `match` 都会立刻编译失败，编译器帮你做了一次全项目的影响面扫描。这个好处很容易被 `_ =>` 破坏，见下方易错点。

```rust
#[derive(Debug)]
enum EpisodeOrder { Tvdb, Dvd, Absolute }

// 变体可以带数据：这里用 enum 表达"顺序 + 该顺序下的编号规则"
#[derive(Debug)]
enum EpisodeRef {
    Tvdb { season: u32, episode: u32 },
    Absolute(u32),
    Unknown,
}

fn describe(order: &EpisodeOrder) -> &'static str {
    match order {
        EpisodeOrder::Tvdb => "按电视台播出顺序",
        EpisodeOrder::Dvd => "按 DVD 发售顺序",
        EpisodeOrder::Absolute => "按绝对集数",
        // 这里故意不写 _ => ...：将来加变体会立刻编译报错，等于一份待办清单
    }
}

fn label(r: &EpisodeRef) -> String {
    match r {
        EpisodeRef::Tvdb { season, episode } => format!("S{season:02}E{episode:02}"),
        EpisodeRef::Absolute(n) => format!("#{n:03}"),
        EpisodeRef::Unknown => String::from("未知"),
    }
}

fn main() {
    for order in [EpisodeOrder::Tvdb, EpisodeOrder::Dvd, EpisodeOrder::Absolute] {
        println!("{:?} -> {}", order, describe(&order));
    }
    println!("{}", label(&EpisodeRef::Tvdb { season: 1, episode: 1 })); // S01E01
    println!("{}", label(&EpisodeRef::Absolute(366)));                  // #366
}
```

⚠️ 三个坑：第一，漏写一个变体会报 **non-exhaustive patterns**，编译器会明确点名缺哪个变体——**别急着用 `_ =>` 把它压下去**，`_` 会让新增变体时不再报错，白白丢掉这道检查，变体少时宁可写全；第二，match 的分支是**按顺序、第一个命中生效**，所以 `_` 之类的宽泛分支必须放最后，放前面会让后面的分支永远不可达（编译器会警告 `unreachable pattern`）；第三，`match order { ... }` 里匹配的是 `&EpisodeOrder`，写 `EpisodeOrder::Tvdb` 就能对上是因为**匹配人体工学**：解构时会自动加引用，不用写成 `&EpisodeOrder::Tvdb`。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| enum 变体只能是名字，不能带数据 | 可以带，而且每个变体携带的数据类型可以完全不同（`Tvdb { ... }` / `Absolute(u32)` / `Unknown` 无数据） |
| 变体名全局唯一 | 不同 enum 可以有同名变体，靠 `EpisodeOrder::Tvdb` 全路径区分 |
| `match` 最后一定要写 `_` 兜底 | 不必。写全所有变体才能吃到穷尽性检查；`_` 是"我不想被编译器管"的信号 |
| enum 值可以直接当数字用（像 C） | 只有 unit 变体能 `as u8`；带数据的变体转不了。Rust 的 enum 默认不保证判别值的具体数字 |

**在 tmdb-organizer 里**：第 12 周会从 TMDB 的 episode group 响应里读出 `type` 字段（`"absolute"` / `"dvd"` / `"tvdb"`），映射成 `EpisodeOrder`——这个映射天生就是 `match`，而且**不能写 `_`**：将来 TMDB 新增一种顺序类型时，我们要靠编译错误被提醒，而不是静默当成 `Unknown` 处理。

---

<a id="w2"></a>

## 第 2 周：所有权、借用、切片、String vs &str

| 天   | 学 6 min                                        | 写 10 min                                                    | 验收 / commit          |
| ---- | ----------------------------------------------- | ------------------------------------------------------------ | ---------------------- |
| D8   | 所有权三条规则与 move（[书 §4.1](https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html)） | `let s1 = String::from("Bleach"); let s2 = s1;` 尝试打印 `s1` 看报错 | 理解 move，`day08`     |
| D9   | 不可变引用与借用（[书 §4.2](https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html)） | `fn print_name(name: &str)`，调用 `print_name(&file.name)`   | 不转移所有权，`day09`  |
| D10  | 可变引用 `&mut` 与借用规则（[书 §4.2](https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html)） | `fn add_episode(file: &mut AnimeFile)`，让 episode +1        | 修改成功，`day10`      |
| D11  | 字符串切片 `&str`（[书 §4.3](https://kaisery.github.io/trpl-zh-cn/ch04-03-slices.html)） | `fn first_word(s: &str) -> &str`，从 `Bleach.S01E01.mkv` 返回 `Bleach` | 打印结果，`day11`      |
| D12  | `String` 与 `&str` 的区别（[书 §8.2](https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html)） | 写 `fn parse_filename(name: &str) -> Option<(u32, u32)>` 初版 | 解析 `S01E01`，`day12` |
| D13  | `Option<T>` 的 `Some` / `None`（[书 §6.1](https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html)） | 让 `parse_filename` 对 `Bleach - 001.mkv` 返回 `None`        | 打印 `Option`，`day13` |
| D14  | 复盘：借用检查器报错怎么读                      | 用 `parse_filename` 解析 3 个文件名                          | `cargo run`，`day14`   |

### D8 · 所有权三条规则与 move

📖 [书 §4.1](https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html)

**三条规则其实只有一句话。** ①每个值有且只有一个主人（owner）；②同一时刻只能有一个主人；③主人离开作用域，值就被释放。这三条合起来就是 Rust 既不靠 GC、也不用你自己 `free` 的全部秘密：**释放时机由作用域决定**，而不是由运行时扫描决定。于是"忘记释放"和"重复释放"这两类 bug 在语法层面就不可能发生。

**move 不是拷贝，是移交。** `let s2 = s1;` 对 `String` 来说做的是**移交所有权**：堆上那段字节一个都没复制，改的只是"谁负责释放它"这个标记。编译器随后把 `s1` 标记为失效，再用就报错。为什么不干脆复制一份？因为如果默认深拷贝，`let s2 = s1;` 就会在一段长字符串上悄悄产生一次分配——这正是别的语言里最难发现的性能问题。

**为什么 `u32` 不受影响。** 判据是 `Copy`：实现了 `Copy` 的类型（所有整数、浮点、`bool`、`char`，以及全由它们组成的元组/数组）赋值时是**按位复制**，原变量照旧可用；`String`、`Vec<T>`、`Box<T>` 拥有堆内存，没有实现 `Copy`，赋值就是 move。一句话判定法：**能用 `memcpy` 安全复制的值才 Copy，持有堆指针的不 Copy**。

| 类型 | `let b = a;` 之后 | 原因 |
| ---- | ---- | ---- |
| `u32` / `bool` / `char` | `a` 仍可用（复制） | 实现了 `Copy`，栈上按位复制 |
| `String` | `a` 失效（move） | 拥有堆内存，两个主人会双重释放 |
| `(u32, u32)` | `a` 仍可用 | 成分全 `Copy`，整个元组也 `Copy` |

**传进函数也是 move。** 把 `String` 作为参数传进去，所有权就进了函数体，函数结束它就 drop。想让调用方继续用，要么**把值返回出来**，要么一开始就传引用（D9 的主题）。

```rust
fn takes_ownership(s: String) -> String {
    println!("函数内看到：{s}");
    s // 无分号 → 把所有权交回调用方
}

fn main() {
    let s1 = String::from("Bleach"); // s1 是 "Bleach" 唯一的主人
    let s2 = s1;                     // move：主人变成 s2
    println!("{s2}");                // 仍能打印：Bleach
    // println!("{s1}");             // 取消注释：borrow of moved value: `s1`

    let n1 = 366;                    // u32 实现了 Copy
    let n2 = n1;                     // 复制，不是 move
    println!("{n1} {n2}");           // 两个都还能用：366 366

    let taken = takes_ownership(s2); // s2 被移进函数
    println!("{taken}");             // 函数把它交回来：Bleach
    // println!("{s2}");             // 取消注释：borrow of moved value: `s2`
}
```

⚠️ 三个坑：第一，`let s2 = s1;` 之后再用 `s1`，报 `borrow of moved value: `s1`（E0382），注意 `-->` 指向的是**使用 `s1` 那一行**，旁边注明 `value borrowed here after move`，根因却在赋值那一行；第二，把一个 `String` 作为参数传进函数后，函数外面再用同样报 E0382——move 之后"原来的名字"不再指向任何东西；第三，别把"失效"理解成"变成空值"：失效是**编译期**的事，运行时并没有把 `s1` 里的指针改掉，只是编译器禁止你再用它。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| move 会复制一份内存 | 只搬"所有权"这个标记，堆数据一个字节都不动，`let s2 = s1` 是 O(1) |
| 所有类型赋值都会 move | 只有非 `Copy` 类型才 move；整数、`bool`、`char` 是按位复制 |
| 被 move 的变量会变成空值 | 是编译期失效，不是运行期置空；编译器直接禁止后续使用 |
| 函数拿走所有权就等于释放了 | 不一定。返回出去所有权就活下来；真正释放发生在最后一个主人离开作用域时 |

**在 tmdb-organizer 里**：第 8 周 `scan_dir` 拿到的一堆 `PathBuf` 会被 move 进 `AnimeFile` 的 `path` 字段——放进去之后原变量就不该再用。这条规则现在看着麻烦，等第 14 周真的改文件名时才会体会到好处：**任何一个文件的路径，永远只有一个负责人**。

### D9 · 不可变引用与借用

📖 [书 §4.2](https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html)

**借用的目的是"用而不拿"。** 如果每次调用都靠 move 传参再用返回值还回来，代码会变成一长串 `let x = f(x);`。引用 `&T` 让函数能**读**到原数据，却不接管释放责任：借用期间主人不变，函数返回后原变量照旧可用。代价是借用要守规则，而这条规则由借用检查器在**编译期**强制执行——不通过就编不出来。

**`&String` 和 `&str` 不是同一个类型，但能自动"降级"。** `&String` 会通过 `Deref` 强制转换（deref coercion）变成 `&str`，所以 `fn print_name(name: &str)` 既能收字面量 `"Bleach"`，也能收 `&file.name`。这就是 D3 那条"参数优先写 `&str`"的底层原因：写 `&str` 相当于同时接纳了两种输入，调用方不用 `clone`，也不用让出所有权。

**借用非常便宜。** `&str` 是"指针 + 长度"两个机器字（胖指针），`&AnimeFile` 就是一个机器字；借用一个结构体不会复制它的任何字段。所以 `&self` 方法、`&str` 参数这类写法是**零拷贝**——不是因为编译器做了优化，而是它压根没复制任何数据。

```rust
struct AnimeFile {
    name: String,
    season: u32,
    episode: u32,
}

fn print_name(name: &str) { // 只借，不拿
    println!("文件：{name}");
}

fn label(f: &AnimeFile) -> String { // 只读借用整个结构体
    format!("S{:02}E{:02}", f.season, f.episode)
}

fn main() {
    let file = AnimeFile {
        name: String::from("Bleach"),
        season: 1,
        episode: 1,
    };

    print_name(&file.name);                 // &String → &str：自动转换
    println!("{}", label(&file));           // 借用 file：S01E01
    println!("借用后仍能用：{}", file.name); // 所有权还在：Bleach
}
```

⚠️ 三个坑：第一，忘了写 `&` 直接传 `file.name`，报 `mismatched types: expected `&str`, found `String`（E0308）——`Deref` 只在**已经有引用**时才帮忙，`String` 本身不会自动变成 `&str`；第二，`label(file)` 少写 `&` 报 `expected `&AnimeFile`, found `AnimeFile`，这条报错比第一条更值得看，它明确告诉你"缺一层借用"；第三，想借的时候顺手改数据，必须先 `let mut file`，否则先报 `cannot borrow `file` as mutable, as it is not declared as mutable`，就算加了 `mut`，只要旧的只读借用还活着，也会报 `cannot borrow `file` as mutable because it is also borrowed as immutable`（E0502，下一天展开）。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 借用就是把值复制一份再传进去 | 传的是地址（可能附带长度），没有任何数据复制 |
| 借用会延长原值的生命周期 | 借用不能超过主人活着的时间，正好相反：是主人在"托着"借用 |
| `&String` 和 `&str` 完全等价 | 是两个类型，只在需要 `&str` 的地方自动转换；`&mut String` 不会自动变成 `&mut str` |
| 借用检查器只管函数边界 | 任何地方都管：`let r = &file.name;` 之后对 `file` 的任何冲突操作都会被拦 |

**在 tmdb-organizer 里**：`parse_filename(name: &str) -> Option<(u32, u32)>` 就是这条规则的模板——只借文件名看一眼，绝不把调用方的字符串吞掉；而第 8 周 `scan_dir` 遍历目录时，每个候选文件也是先 `&path` 借给解析函数试一下，解析成功才决定要不要 move 进 `AnimeFile`。

### D10 · 可变引用 `&mut` 与借用规则

📖 [书 §4.2](https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html)

**`&mut` 的准确含义是"独占访问权"。** 它不只表示"可改"，还表示**排他**：拿到 `&mut` 的这段时间里，任何人都不许再读也不许再写这块数据。`&T`（共享借用）相反，可以同时存在任意多个，但每一个都只能读。所以"可写"的代价是"必须只有一个"。

**为什么只允许一个 `&mut`。** 因为这一条把**数据竞争**从运行期问题变成了编译期错误。数据竞争要同时满足"两处访问 + 至少一处写 + 没有同步"，而"要么多个只读、要么一个可写"这条规则恰好精确排除了它。别的语言要靠约定和 code review 防，Rust 直接让你编不过——D18 讲闭包、第 10 周讲多线程时，你会反复用上这条。

**借用的终点是"最后一次使用"，不是作用域末尾。** 这叫非词法生命周期（NLL）：`let a = &mut file; a.episode += 1;` 之后 `a` 就到头了，再 `&mut file` 完全合法。所以看到"两个可变借用冲突"时，实际判据是**两个借用都还被使用**——不用的那个不算数。

| 组合 | 是否允许 | 冲突时报什么 |
| ---- | ---- | ---- |
| 多个 `&T` | 允许 | 只读，互不干扰 |
| 一个 `&mut T` | 允许 | 独占，正常 |
| `&mut T` + `&T` | **不允许** | E0502 |
| 两个 `&mut T` | **不允许** | E0499 |

```rust
struct AnimeFile {
    name: String,
    season: u32,
    episode: u32,
}

fn add_episode(f: &mut AnimeFile) { // 可变借用：可以改字段
    f.episode += 1;
}

fn main() {
    let mut file = AnimeFile {
        name: String::from("Bleach"),
        season: 1,
        episode: 1,
    };

    add_episode(&mut file);                                     // 1 → 2
    println!("{} S{}E{}", file.name, file.season, file.episode); // Bleach S1E2

    // 一个可变借用用完（NLL），才可以再借第二个
    {
        let a = &mut file;
        a.episode += 1;                                         // 2 → 3
    }                                                           // 借用在这里结束
    add_episode(&mut file);                                     // 3 → 4
    println!("{}", file.episode);                               // 4
}
```

⚠️ 三个坑：第一，同一时刻借两次可变，报 `cannot borrow `f` as mutable more than once at a time`（E0499），`-->` 指向**第二次**借用，下面的 `note` 才告诉你第一次借在哪；第二，变量少写 `mut`，报 `cannot borrow `f` as mutable, as it is not declared as mutable`（E0596）——"能借出可变引用"的前提是变量本身是 `let mut`；第三，边遍历边改会撞 E0502：`for f in &files` 期间 `files.push(...)` 报 `cannot borrow `files` as mutable because it is also borrowed as immutable`，因为迭代器正握着不可变借用。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 写了 `&mut` 就能随便借几次 | 同一时间只能有一个，第二次借要等第一个的最后一次使用之后 |
| 加 `mut` 就能解决所有借用报错 | `mut` 只解决 E0596；E0499/E0502 要靠调整借用的范围和时间 |
| 借用冲突是作用域级别的 | NLL 之后按**最后一次使用**判定，用 `{}` 收窄范围是最常用的修法 |
| `&mut` 和 `&` 只是"可写/不可写"的区别 | 还有**数量**区别：`&` 可多个，`&mut` 必须唯一，这才是并发的安全基础 |

**在 tmdb-organizer 里**：真正需要 `&mut` 的地方很有限，这本身就是个信号——第 8-14 周只有两类：统计/累加（如"给每个文件的 episode 编号 +1"）和阶段性的中间状态（如第 12 周往 `HashMap` 里插入绝对集数映射）；而改文件名走的是 `fs::rename`，不需要任何 `&mut`。

### D11 · 字符串切片 `&str`

📖 [书 §4.3](https://kaisery.github.io/trpl-zh-cn/ch04-03-slices.html)

**切片是"视图"，不是新字符串。** `&s[..i]` 不会复制任何字节，它只是记录了"指向 `s` 内部从 0 到 i 这一段"（指针 + 长度）。内存里仍然只有一份 `"Bleach.S01E01.mkv"`，切片是它的一个取景框。也正因如此，切片必须挂在原数据上：原数据一旦变动（比如 `push_str` 触发了重新分配），取景框就指向了废弃的地址。

**`&str` 就是这个取景框类型。** 字符串字面量 `"Bleach"` 的类型就是 `&str`——它指向程序二进制里的一段静态内存，天然满足"指向别处的 UTF-8 字节"。所以"字面量"和"从 `String` 上切下来的一段"是**同一个类型** `&str`，函数签名写 `&str` 就能同时接受两者。这也是 Rust 字符串 API 几乎清一色用 `&str` 做参数的原因。

**下标按字节算，但必须落在字符边界上。** `String` 内部是 UTF-8，`&s[0..1]` 里的 `0` 和 `1` 是**字节**偏移。落在 ASCII 上没问题，切在多字节字符中间编译能过、运行会 panic。中文一个字占 3 字节，所以处理中文文件名时别用固定下标，用 `find`、`split`、`char_indices` 这类返回"安全位置"的方法。

```rust
fn first_word(s: &str) -> &str {
    // find 返回的字节下标天然落在字符边界上
    match s.find('.') {
        Some(i) => &s[..i], // 取景框：不复制，指向 s 的前 i 个字节
        None => s,
    }
}

fn main() {
    let filename = String::from("Bleach.S01E01.mkv");
    let name = first_word(&filename);    // 借走 filename 的一段
    println!("{name}");                  // Bleach
    println!("原串仍然可用：{filename}");  // Bleach.S01E01.mkv

    let literal: &str = "Bleach";        // 字面量本身就是 &str
    println!("{}", first_word(literal)); // Bleach
}
```

⚠️ 三个坑：第一，切在中文中间不是编译错误，而是运行期 panic：`byte index 1 is not a char boundary; it is inside '死' (bytes 0..3) of `死神.Bleach`——这类问题测试一漏就带上线；第二，手里握着切片时原串不能改，`owned.clear()` 会报 `cannot borrow `owned` as mutable because it is also borrowed as immutable`（E0502，前提是切片在后面还被用）；第三，切片不能凭空产生：`fn label() -> &str { format!("S01E01").as_str() }` 报 `missing lifetime specifier`（E0106），因为函数内部造出的 `String` 一出函数就被释放，切片无处可指——**返回 `&str` 只能"指向别人给的东西"**（第 40 天会正式讲这背后的生命周期规则）。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 切片是原字符串的副本 | 是视图，长度和指针两个字；改原串会让切片失效（编译期被拦） |
| `&s[..]` 和 `s.clone()` 差不多 | 前者零拷贝、指向原数据；后者分配新内存、独立存在 |
| 下标是字符序号 | 是**字节**偏移。中文占 3 字节，`s[0..3]` 才是一个完整的字 |
| `find` 返回字符下标 | 返回**字节**下标（`Option<usize>`），正因为它返回的是字节位置，才天然安全 |

**在 tmdb-organizer 里**：`parse_filename` 从 `Bleach.S01E01.mkv` 里取 `S01E01` 那一段时，先 `find('S')` 拿字节位置再切片，全程零拷贝；只有最后 `parse::<u32>()` 把数字转成整数时才真正产生了新值——这就是"**能在原数据上看的，就不要复制**"。

### D12 · `String` 与 `&str` 的区别

📖 [书 §8.2](https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html)

**一个是"拥有"，一个是"借用"。** `String` 是三个机器字：指针、长度、容量——它拥有堆上那段数据，并且可以增长（`push_str` 追容量不够时会重新分配、搬走）；`&str` 只有指针和长度两个字，**永远不能改**，只能指给别人看。所以判断标准很直接：需要造新字符串、需要增长，用 `String`；只是看一眼，用 `&str`。

**API 为什么偏爱 `&str`。** 因为 `&str` 是最宽的输入类型：字面量、`&String`（自动 deref）、`String` 的切片都能传进来。函数如果写 `fn f(s: String)`，调用方就必须交出一个拥有的字符串，或者 `.clone()` 一次，白付一次分配。反过来**返回**时，如果内容是函数里新造的，那只能返回 `String`，因为那份数据需要一个主人。

**两个方向的转换，一个花钱一个免费。** `&str → String`：`.to_string()` 或 `String::from(...)`，分配一次堆内存。`String → &str`：`&s` 或 `s.as_str()`，零开销，只是加了个借用的标记。记牢：**往上（借用 → 拥有）要花钱，往下（拥有 → 借用）免费。**

| 表达式 | 类型 | 是否分配 |
| ---- | ---- | ---- |
| `"Bleach"` | `&'static str` | 不分配，指向二进制里的静态数据 |
| `String::from("Bleach")` | `String` | 堆上分配一次 |
| `s.as_str()` / `&s` | `&str` | 不分配，仅借用 |
| `format!("{s}.mkv")` | `String` | 每次调用都分配 |

```rust
fn parse_filename(name: &str) -> Option<(u32, u32)> {
    // 期待形如 "Bleach.S01E01.mkv"：先定位 S 与 E
    let (s_pos, e_pos) = match (name.find('S'), name.find('E')) {
        (Some(s), Some(e)) => (s, e),
        _ => return None,
    };
    if e_pos <= s_pos {
        return None;
    }
    let season = name[s_pos + 1..e_pos].parse::<u32>().ok(); // &str → Option<u32>
    let tail = &name[e_pos + 1..];                           // 借来的一段视图
    let cut = tail.find('.').unwrap_or(tail.len());           // 找不到就取到最后
    let episode = tail[..cut].parse::<u32>().ok();
    match (season, episode) {
        (Some(s), Some(e)) => Some((s, e)),
        _ => None,
    }
}

fn main() {
    let owned = String::from("Bleach.S01E01.mkv"); // String：拥有堆数据
    let view: &str = &owned;                       // &str：只是借用，零拷贝
    println!("{:?}", parse_filename(view));        // Some((1, 1))
    println!("{:?}", parse_filename("Bleach - 001.mkv")); // None
    println!("{owned}");                           // 原 String 仍可用
}
```

⚠️ 三个坑：第一，`let s: &str = String::from("Bleach");` 报 `mismatched types: expected `&str`, found `String`（E0308），要写成 `let owned = String::from(...); let s: &str = &owned;`；第二，`a + &b` 拼接会 move 掉 `a`（`+` 的签名本质上是 `fn add(self, other: &str) -> String`），之后再打印 `a` 报 `borrow of moved value: `a`（E0382），想保留就写 `format!("{a}{b}")`；第三，`len()` 返回**字节数**不是字符数：`"死神.Bleach".len()` 是 13，`chars().count()` 才是 10，用它做"文件名是否超长"的判断会算错。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `&str` 是 `String` 的简化版 | 是两种不同的东西：`String` 拥有堆内存，`&str` 是借用；生命周期完全不同 |
| 函数参数写 `String` 更"通用" | 更窄：调用方被迫交出所有权或 `clone`。参数优先 `&str`，要存起来才收 `String` |
| `.to_string()` 只是类型转换，没有成本 | 有一次堆分配；不需要新字符串时（只读）不要调它 |
| `String` 就是 `Vec<u8>` | 它确实是 UTF-8 字节的容器，但额外保证**内容永远是合法 UTF-8**，所以不能随便塞任意字节 |

**在 tmdb-organizer 里**：整条解析链都遵守"进 `&str`、出 `String`"——`parse_filename(name: &str) -> Option<(u32, u32)>` 只借；`display_label(&self) -> String` 则必须造新的（`format!` 的结果没有别的归属）。第 14 周真正改名时，目标文件名也是一个新建的 `String`，交给 `fs::rename` 用一下即可。

### D13 · `Option<T>` 的 `Some` / `None`

📖 [书 §6.1](https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html)

**没有 null，只有"可能没有值"这个类型。** `Option<T>` 就是标准库里一个普通枚举：`enum Option<T> { Some(T), None }`。它把"可能为空"**编进类型**：`Option<u32>` 和 `u32` 是两个不同类型，想拿到里面的 `u32`，就必须先处理 `None` 那条路。别的语言的 null 是"任何类型都可能空"，编译器帮不上忙；Rust 是"只有 `Option` 会空"，于是编译器能替你检查全部路径。

**它常常不占额外空间。** 对 `Option<&T>`、`Option<Box<T>>` 这类内部含指针的类型，编译器会用"空指针"表示 `None`——这叫**空指针优化**，所以 `Option<&T>` 和 `&T` 一样大。但 `Option<u32>` 没有可榨取的无效位，必须额外带一个判别字。这也是 Rust 敢用 `Option` 表达"可能没有"而不担心性能的原因。

**三种处理方式，按场景挑。** `match` 最完整，两个分支都写、能拿到值；`if let Some(x) = ...` 只关心"有值"的情况（D33 正式讲）；`unwrap()` / `expect()` 表示"我确信不是 `None`，否则崩"，只该用在内部不变式或原型里；`?` 运算符（第 4 周）最常用——`None` 直接提前返回。

```rust
fn parse_filename(name: &str) -> Option<(u32, u32)> {
    // 形如 "Bleach.S01E01.mkv"：先定位 S 与 E
    let (s_pos, e_pos) = match (name.find('S'), name.find('E')) {
        (Some(s), Some(e)) => (s, e),
        _ => return None,
    };
    if e_pos <= s_pos {
        return None;
    }
    let season = match name[s_pos + 1..e_pos].parse::<u32>() {
        Ok(n) => n,
        Err(_) => return None, // 解析失败也走 None，不 panic
    };
    let tail = &name[e_pos + 1..];
    let cut = match tail.find('.') {
        Some(i) => i,
        None => tail.len(),
    };
    let episode = match tail[..cut].parse::<u32>() {
        Ok(n) => n,
        Err(_) => return None,
    };
    Some((season, episode))
}

fn main() {
    for filename in ["Bleach.S01E01.mkv", "Bleach.S01E366.mkv", "Bleach - 001.mkv"] {
        match parse_filename(filename) {
            Some((s, e)) => println!("{filename} -> S{s:02}E{e:02}"),
            None => println!("{filename} -> 无法解析"),
        }
    }
}
```

⚠️ 三个坑：第一，`match` 只写 `Some` 不写 `None`，报 `non-exhaustive patterns: `None` not covered`（E0004）——`Option` 是普通枚举，穷尽性检查一样管它；第二，想直接解构 `let (s, e) = parse_filename(...)` 报 `mismatched types: expected `Option<(u32, u32)>`, found `(_, _)`（E0308），必须先 `match` 出 `Some`；第三，`None.unwrap()` 编译能过，运行期 panic `called `Option::unwrap()` on a `None` value`，`-->` 直接指向 `unwrap` 那一行——**`unwrap` 是把"可能没有"重新变回"必然有"的赌注**，赌错了就崩。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `Option<T>` 是个特殊的语言特性 | 是标准库里一个普通枚举，和 D7 的 `EpisodeOrder` 是同一套机制 |
| 用 `Option` 会拖慢性能 | 含指针的 `Option` 走空指针优化，大小不变；其余情况多一个判别字 |
| `unwrap()` 是"安全取出" | 它是"`None` 就 panic"，混在解析逻辑里会让整批文件处理直接中止 |
| `Some(1)` 和 `1` 能互相赋值 | 两个不同类型，`let x: u32 = Some(1);` 报 E0308，必须 `match`/`unwrap` 才拿到里层值 |

**在 tmdb-organizer 里**：`parse_filename(name: &str) -> Option<(u32, u32)>` 的返回类型就是这一天的成果——目录里必然混着 `Bleach - 001.mkv`、`sample.mkv`、`Cover.jpg` 这类解析不了的条目，用 `None` 表示"跳过"，配第 8 周的 `filter_map` 就能把无效文件和有效文件一次分开，而且**不会因为一个坏文件名崩掉整次扫描**。

### D14 · 复盘：借用检查器报错怎么读

📖 [书 §4.1](https://kaisery.github.io/trpl-zh-cn/ch04-01-what-is-ownership.html)、[书 §4.2](https://kaisery.github.io/trpl-zh-cn/ch04-02-references-and-borrowing.html)

**报错是"三段式"，要从下往上读。** 一个典型的借用错误长这样：第一行是错误码加一句结论（`cannot borrow ... as mutable more than once`），中间 `-->` 指向**违规那一点**（第二个借用在哪），最后 `note: ... first mutable borrow occurs here` 指向**根因**（第一个借用在哪）。`-->` 只是"出事故的位置"，`note` 才是"为什么会冲突"——先看 `note` 找第一个借用，再看违规点，十秒钟就能定位。

**四个错误码覆盖九成报错。** `E0382 moved value`（值被搬走了）、`E0499 twice mutable`（两个可变借用）、`E0502 immutable/mutable 冲突`（读借用还没用完就要写）、`E0596 not declared as mutable`（漏写 `mut`）。看到错误码先归到哪一类，修法基本就固定了：E0596 加 `mut`；其余三个都是在"缩短借用时间"或"改成借用"。

**修法的优先顺序。** ①先问**是不是根本不需要这么早借**——把借用挪到使用处、或用 `{}` 把借用圈在小范围里；②再问能不能"先把要用的数据算出来（或复制一个 `u32` 这样的小值），再去改原结构"；③`clone()` 是最后手段，它能骗过借用检查器，但每次调用都是真实分配——**能过编译不等于写法好**。

```rust
#[derive(Debug)]
struct AnimeFile {
    name: String,
    season: u32,
    episode: u32,
}

fn label(file: &AnimeFile) -> String { // 只读借用
    format!("{} S{:02}E{:02}", file.name, file.season, file.episode)
}

fn rename(file: &mut AnimeFile, new_name: &str) { // 可变借用
    file.name = new_name.to_string();
}

fn main() {
    let mut file = AnimeFile {
        name: String::from("Bleach.S01E01.mkv"),
        season: 1,
        episode: 1,
    };

    // 错误一：move 之后再用（E0382）
    // let moved = file.name;      // String 被搬走
    // println!("{}", file.name);  // borrow of moved value

    // 修法：借用而不是搬走
    println!("{}", label(&file)); // Bleach.S01E01.mkv S01E01

    // 错误二：同时存在两个可变借用（E0499）
    // let a = &mut file;
    // let b = &mut file;          // cannot borrow as mutable more than once

    // 修法：第一个用完之后再借第二个
    rename(&mut file, "Bleach.S01E02.mkv");
    println!("{}", label(&file)); // Bleach.S01E02.mkv S01E01
}
```

⚠️ 三个坑：第一，把 `-->` 当成病根，在那一行反复加 `clone()`——其实 `note` 里指出的第一个借用才是要动的地方；第二，以为"借用冲突 = 作用域重叠"，于是到处加 `{}` 也不见效，正确判据是**借用是否还被使用**（NLL）；第三，`fn rename(file: &mut AnimeFile)` 里 `file.name = new_name.to_string();` 把 `name` 整个换掉，这是合法的；但如果写 `file.name.push_str(new_name)` 之类的"读+写"混用出现冲突，要意识到冲突来自**你自己同时要了两种权限**，而不是编译器刁难。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `-->` 指向的行就是错的那行 | 那只是**违规点**；`note` 里的第一个借用才是根因 |
| 借用检查器只看作用域 | NLL 后按**最后一次使用**判边界，用 `{}` 能收窄但不能解决"还在用" |
| 加 `.clone()` 是标准解法 | 最后手段。先试"缩小借用范围"或"先算完小值再改"，`clone` 有真实分配成本 |
| E0499 和 E0502 是同一个问题 | E0499 是两个可变借用；E0502 是可变与不可变冲突。触发的组合不同 |

**在 tmdb-organizer 里**：这套读法第 8 周就会天天用——`scan_dir` 先借用 `Path` 再产出 `Vec<PathBuf>`，`parse_filename` 借用 `&str` 返回 `Option`，直到第 14 周才出现真正需要 `&mut` 的统计逻辑。养成"先想权限、再想代码"的习惯，后面加 `HashMap`、闭包、多线程时才能一次写对。

---

<a id="w3"></a>

## 第 3 周：Vec、String、HashMap、闭包、迭代器

| 天   | 学 6 min                                    | 写 10 min                                               | 验收 / commit               |
| ---- | ------------------------------------------- | ------------------------------------------------------- | --------------------------- |
| D15  | `Vec<T>` 创建、`push` 与遍历（[书 §8.1](https://kaisery.github.io/trpl-zh-cn/ch08-01-vectors.html)） | `let mut files: Vec<AnimeFile> = vec![];` push 3 个文件 | 打印 `files.len()`，`day15` |
| D16  | `String` 与 `format!` 补零（[书 §8.2](https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html)） | 用 `format!` 生成 `S{:02}E{:02}`                        | 生成 label，`day16`         |
| D17  | `HashMap<K, V>` 基本操作（[书 §8.3](https://kaisery.github.io/trpl-zh-cn/ch08-03-hash-maps.html)） | `HashMap<u32, (u32, u32)>` 存 Absolute→TVDB 映射        | 插入 3 条，`day17`          |
| D18  | 闭包与环境捕获（[书 §13.1](https://kaisery.github.io/trpl-zh-cn/ch13-01-closures.html)） | 用闭包按 episode 排序 `Vec<AnimeFile>`                  | 打印排序后，`day18`         |
| D19  | 迭代器与惰性求值（[书 §13.2](https://kaisery.github.io/trpl-zh-cn/ch13-02-iterators.html)） | `files.iter().filter(\|f\| f.season == 1).count()`      | 打印数量，`day19`           |
| D20  | 迭代器适配器 `map` / `collect`（[书 §13.3](https://kaisery.github.io/trpl-zh-cn/ch13-03-improving-our-io-project.html)） | `map` 收集所有 `display_label` 到 `Vec<String>`         | 打印列表，`day20`           |
| D21  | 复盘：把 `for` 循环改写成链式调用                | 用迭代器生成一份简单文本报告                            | `cargo run`，`day21`        |

### D15 · `Vec<T>` 创建、`push` 与遍历

📖 [书 §8.1](https://kaisery.github.io/trpl-zh-cn/ch08-01-vectors.html)

**`Vec<T>` 就是"可增长的数组"。** 内存里它是三个机器字：指针、长度、容量。长度是"现在有几个元素"，容量是"已经申请了能放几个"。`push` 时一旦长度追上容量，它就申请一块更大的内存（通常是翻倍）、把元素搬过去、再更新指针——所以 `push` 绝大多数时候是 O(1)，偶尔是 O(n)，而 `len()` 永远是 O(1)。"容量"这个概念存在的唯一目的，就是把偶发的搬家成本**摊平**到每次 `push` 上。

**为什么元素必须同类型。** 所有元素类型相同、大小相同，`v[i]` 才能直接用"指针 + i × 元素大小"算出来，常数时间。要放"类型不同"的东西，就得靠 D7 学会的带数据枚举，或者第 6 周的 trait 对象——**用一层包装换回同构**。这同时也解释了 `Vec` 为什么比 `HashMap` 快：没有哈希，只有乘法和加法。

**三种遍历，语义完全不同。** `for f in &files` 借出 `&AnimeFile`，只看不动；`for f in &mut files` 借出 `&mut AnimeFile`，可以改元素；`for f in files` 会把整个 `Vec` **move 掉**，循环之后 `files` 就失效了。取单个元素也有两种：`files[i]` 越界直接 panic，`files.get(i)` 返回 `Option<&AnimeFile>`——前者表示"我确信下标有效"，后者表示"我要处理越界"。

| 写法 | 每次拿到 | 循环后 `files` 还能用吗 |
| ---- | ---- | ---- |
| `for f in &files` | `&AnimeFile` | 能 |
| `for f in &mut files` | `&mut AnimeFile` | 能（元素可能已被改） |
| `for f in files` | `AnimeFile`（move） | **不能**，已被消费 |

```rust
#[derive(Debug)]
struct AnimeFile {
    name: String,
    season: u32,
    episode: u32,
}

fn main() {
    // vec![] 造空 Vec，元素类型由注解决定
    let mut files: Vec<AnimeFile> = vec![];

    files.push(AnimeFile { name: String::from("Bleach"), season: 1, episode: 1 });
    files.push(AnimeFile { name: String::from("Bleach"), season: 1, episode: 2 });
    files.push(AnimeFile { name: String::from("Bleach"), season: 1, episode: 3 });

    println!("共 {} 个文件", files.len()); // 共 3 个文件

    // 只读遍历 &files，拿到的是 &AnimeFile
    for f in &files {
        println!("S{:02}E{:02}", f.season, f.episode);
    }

    // 下标访问和数组一样，越界会 panic
    println!("第 1 个是 {}", files[0].name); // 第 1 个是 Bleach
}
```

⚠️ 三个坑：第一，`let files = vec![];` 后面又没有 `push` 过任何元素，报 `type annotations needed for `Vec<_>`（E0282）——空的 `Vec` 推不出元素类型，写成 `let mut files: Vec<AnimeFile> = vec![];` 或在 `push` 处让类型可见即可；第二，`for f in &files` 的循环体里 `files.push(...)`，报 `cannot borrow `files` as mutable because it is also borrowed as immutable`（E0502），因为迭代器正握着 `files` 的不可变借用；第三，`for f in files`（少写 `&`）之后再 `files.len()`，报 `borrow of moved value: `files`（E0382）——**漏一个 `&` 就是把数据送走了**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `Vec` 就是链表 | 是**连续内存**的数组，`v[i]` 是 O(1)；`push` 只在扩容时才搬一次家 |
| `push` 每次都重新分配 | 按容量翻倍增长，平均每次 `push` 是摊还 O(1) |
| `files.len()` 是元素个数也是容量 | `len()` 是**元素个数**，`capacity()` 是已申请的空间，两者经常不等 |
| `v[i]` 越界会返回 `None` | 直接 panic。要安全访问得用 `v.get(i)`，它才返回 `Option<&T>` |

**在 tmdb-organizer 里**：第 8 周 `scan_dir` 的返回类型就是 `Vec<PathBuf>`，第 8-14 周的所有处理都是"扫出一批 → 过滤一批 → 生成一批"，中间产物清一色是 `Vec<AnimeFile>` 和 `Vec<RenamePlan>`；`Vec::len()` 还会用来做"扫到几个文件"的日志。

### D16 · `String` 与 `format!` 补零

📖 [书 §8.2](https://kaisery.github.io/trpl-zh-cn/ch08-02-strings.html)

**`format!` 就是"造 `String` 的 `println!`"。** 格式化语法完全一样，区别只在结果：`println!` 写进标准输出、返回 `()`；`format!` 返回一个新的 `String`。这也意味着 `format!` **必定有一次堆分配**——在"每个文件都要生成新名字"的路径上这是必要的开销，但在纯日志打印路径上应改用 `println!`，别白白多造一个 `String`。

**`{:02}` 的三部分怎么读。** `:` 后面是**格式说明**：第一个 `0` 表示"用 `0` 填充"，`2` 表示"最小宽度为 2"。于是 `1` 变 `01`、`7` 变 `07`，而 `366` 还是 `366`——**宽度是下限，不会截断**。`{:>6}` 则是右对齐、空格填充（`>` 是方向，`6` 是宽度）。这套语法在 `format!`、`println!`、`write!` 里全部通用，学会一次到处能用。

| 写法 | `1` 的输出 | `366` 的输出 | 说明 |
| ---- | ---- | ---- | ---- |
| `{:02}` | `01` | `366` | 补 `0` 到 2 位 |
| `{:03}` | `001` | `366` | 补 `0` 到 3 位，绝对集数常用 |
| `{:>6}` | `     1` | `   366` | 空格右对齐到 6 位 |

**为什么整理工具非补零不可。** 文件名排序走的是**字典序**：`E10` 排在 `E2` 前面，因为字符 `'1'` 先于 `'2'`。补零之后 `E02 < E10`，字典序恰好等于数值序，播放器和文件管理器才会按正确顺序排。这是 D16 最实用的一条原则：**只要机器排序要和人眼顺序一致，就必须定宽补零。**

```rust
#[derive(Debug)]
struct AnimeFile {
    name: String,
    season: u32,
    episode: u32,
}

impl AnimeFile {
    fn label(&self) -> String {
        // {:02}：宽度不足 2 时补 0
        format!("S{:02}E{:02}", self.season, self.episode)
    }

    fn padded_absolute(&self) -> String {
        // 绝对集数补到 3 位，方便按文件名排序
        format!("#{:03}", self.episode)
    }
}

fn main() {
    let f = AnimeFile { name: String::from("Bleach"), season: 1, episode: 7 };
    println!("{}", f.label());           // S01E07
    println!("{}", f.padded_absolute()); // #007

    // 宽度只是下限：位数够了不会截断
    let big = AnimeFile { name: String::from("Bleach"), season: 1, episode: 366 };
    println!("{}", big.label());    // S01E366
    println!("{:>6}|", big.label()); // S01E366|
}
```

⚠️ 三个坑：第一，占位符个数和参数个数必须对上：`println!("S{}E{}", season)` 报 `2 positional arguments in format string, but there is 1 argument`，`-->` 直接指向那个里层宏；第二，内联捕获和多余的位置参数混写，`println!("{name}", "Naruto")` 报 `argument never used`（E0308 之外的另一类"参数对不上"）；第三，`{:02}` 用在**字符串**上不报错也不补零——它对字符串只是"最小宽度 2、空格填充"，`println!("[{:02}]", "a")` 输出 `[a ]`，要补零只能对数字用。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `{:02}` 是"只保留两位" | 是最小宽度，位数超了照原样输出，永不截断 |
| `format!` 和 `println!` 差不多 | `format!` 返回 `String`（有分配），`println!` 返回 `()` 并写 stdout |
| `{:02}` 对字符串也能补零 | 字符串的填充字符是空格，补零只对数字有意义 |
| 文件名排序按数字大小 | 按**字典序**。不定宽补零就会出现 `E10` 排在 `E2` 前面 |

**在 tmdb-organizer 里**：`display_label(&self) -> String` 就是 `format!("S{:02}E{:02}", ...)`；`padded_absolute` 的思路会在第 12 周做绝对集数映射时复用，第 14 周真正落盘的 `Bleach - S01E07.mkv` 也靠它保证本地目录里排序正确。

### D17 · `HashMap<K, V>` 基本操作

📖 [书 §8.3](https://kaisery.github.io/trpl-zh-cn/ch08-03-hash-maps.html)

**它是"哈希表"，代价是无序。** 键先经哈希函数算出桶的位置，所以 `insert` / `get` 平均 O(1)；代价是**遍历顺序不稳定**——同一份数据两次运行打印出来的顺序可能不同。需要按 key 有序就换 `BTreeMap`（O(log n)，但遍历有序）。整理工具里"绝对集数 → (季, 集)"这种按编号查表的场景，`HashMap` 正合适。

**`get` 返回 `Option<&V>`，这是所有权导致的必然。** 键可能不存在，所以返回值必须能表达"没有"；而 `HashMap` 还拥有那个 value，所以只能借给你 `&V`，拿不走。想拿走用 `remove`，想改用 `get_mut`。另外 `get` 的参数类型是 `&Q`，所以必须写 `map.get(&2)`——**整数键要加 `&`**，这是新手最常见的一处小别扭。

**`entry` 是"查不到就插入"的一次查找写法。** `map.entry(k).or_insert(v)` 返回 `&mut V`，所以能接着 `+= 1` 做计数；而 `insert` 是**永远覆盖**并返回旧值的 `Option<V>`。别写 `if !map.contains_key(&k) { map.insert(k, v); }`——那是两次查找，`entry` 只查一次，而且在"你要根据旧值决定新值"时它是唯一自然的写法。

```rust
use std::collections::HashMap;

fn main() {
    // 绝对集数 → (季, 集)
    let mut mapping: HashMap<u32, (u32, u32)> = HashMap::new();

    mapping.insert(1, (1, 1));
    mapping.insert(2, (1, 2));
    mapping.insert(3, (1, 3));
    println!("共 {} 条", mapping.len()); // 共 3 条

    // get 返回 Option<&V>，找不到就是 None
    match mapping.get(&2) {
        Some((s, e)) => println!("第 2 集 -> S{s:02}E{e:02}"), // S01E02
        None => println!("没有第 2 集"),
    }

    // entry + or_insert：不存在才插入
    mapping.entry(4).or_insert((1, 4));
    mapping.entry(1).or_insert((9, 9)); // 1 已存在，不会被覆盖
    println!("第 4 集 -> {:?}", mapping.get(&4)); // Some((1, 4))
    println!("第 1 集 -> {:?}", mapping.get(&1)); // Some((1, 1))
}
```

⚠️ 三个坑：第一，`mapping.get(2)` 少写 `&`，报 `mismatched types: expected `&u32`, found integer`（E0308），编译器通常还会给出 `help: consider borrowing here: `&2`；第二，插入的值类型和声明不一致，`mapping.insert(2, 2)` 报 `mismatched types: expected `(u32, u32)`, found integer`，`-->` 指向第二个参数；第三，以为 `insert` 是"没有才插"——`insert` **永远覆盖**旧值并把它作为 `Option<V>` 返回，"没才插"必须用 `entry().or_insert()`。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `HashMap` 遍历顺序是按插入顺序 | 无序，且同一进程两次建表顺序都可能不同；要顺序就用 `BTreeMap` |
| `get` 返回值的引用，说明键不存在也是这个引用 | 返回 `Option<&V>`，键不存在是 `None`，必须 `match` / `if let` |
| `insert` 和 `entry().or_insert()` 等价 | `insert` 覆盖旧值；`or_insert` 只在没键时插入 |
| `HashMap` 的 key 可以是任何类型 | 必须实现 `Eq` + `Hash`；`f64` 就不行（没有全序），字符串、整数没问题 |

**在 tmdb-organizer 里**：第 12 周会从 TMDB 的 episode group 响应里读出"绝对集数 → 季/集"的映射，装进 `HashMap<u32, (u32, u32)>`，第 13 周把本地文件名里的 `Bleach - 001.mkv` 换算成 `S01E01` 时就是一次 `mapping.get(&1)`；找不到时返回的 `None` 正好用来决定"这个文件跳过并记进日志"。

### D18 · 闭包与环境捕获

📖 [书 §13.1](https://kaisery.github.io/trpl-zh-cn/ch13-01-closures.html)

**闭包 = 函数 + 捕获的环境。** `|f: &AnimeFile| f.episode` 是一个**值**，可以存进变量、传给函数、作为返回值；它比普通函数多出来的能力，是能看见定义处外层的变量。实现上没什么魔法：编译器为每个闭包生成一个匿名结构体，把捕获到的变量当字段装进去，调用时就是"读字段"——所以捕获的本质是**把外部变量搬进结构体**。

**捕获方式由"你用它做什么"推断出来。** 只读就捕获 `&值`（`Fn`），要改就捕获 `&mut 值`（`FnMut`），要拿走（比如存进 `Vec` 或者移出线程）就捕获值本身（`FnOnce`），也可以用 `move` 关键字强制拿走。编译器总是选**权限最小**的那种，好处是闭包能活得更久、能被复用；代价是你在闭包外面对这些变量的操作会被借用检查器盯着。

| 闭包里干了什么 | 捕获到什么 | 闭包类型 |
| ---- | ---- | ---- |
| 只读 `f.season` | `&值` | `Fn` |
| 改 `count += 1` | `&mut 值` | `FnMut` |
| 把值 move 出去 | 值 | `FnOnce` |

**`sort_by` 为什么收闭包。** 排序的比较规则每次都不同，做成固定函数不现实，做成 trait 对象又偏重，所以标准库把它设计成"收一个 `FnMut(&T, &T) -> Ordering`"。于是 `|a, b| a.episode.cmp(&b.episode)` 就是这个参数的最小写法——`cmp` 返回的 `Ordering` 正是排序要的"小于 / 等于 / 大于"。想按季再按集排序时也是同一个位置改一行：`a.season.cmp(&b.season).then(a.episode.cmp(&b.episode))`。

```rust
#[derive(Debug)]
struct AnimeFile {
    name: String,
    season: u32,
    episode: u32,
}

fn main() {
    let mut files = vec![
        AnimeFile { name: String::from("Bleach"), season: 1, episode: 3 },
        AnimeFile { name: String::from("Bleach"), season: 1, episode: 1 },
        AnimeFile { name: String::from("Bleach"), season: 1, episode: 2 },
    ];

    // 闭包捕获外层的 min_season（u32 是 Copy，按值捕获）
    let min_season = 1;
    let belongs = |f: &AnimeFile| f.season >= min_season;
    println!("第 3 集属于目标季吗：{}", belongs(&files[0])); // true

    // 按集号排序，闭包借用两个元素做比较
    files.sort_by(|a, b| a.episode.cmp(&b.episode));
    for f in &files {
        println!("E{:02}", f.episode); // 依次 E01 E02 E03
    }
}
```

⚠️ 三个坑：第一，闭包里要改外部变量，那个闭包变量必须 `let mut`，否则报 `cannot borrow `collector` as mutable, as it is not declared as mutable`（E0596），`-->` 指向**调用闭包**的那一行；第二，闭包捕获了 `&mut files` 之后，外面再读 `files.len()` 报 `cannot borrow `files` as immutable because it is also borrowed as mutable`（E0502）——闭包持着借用，借用要在闭包最后一次使用之后才还回来；第三，闭包参数类型通常能推断，但猜错时编译器会甩出一长串 trait 报错（`expected a closure that implements ...`），所以**给闭包参数显式写类型**（像 `|f: &AnimeFile|`）能显著改善报错质量。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 闭包就是匿名函数 | 多了"捕获环境"，因此有状态；编译器为它生成结构体来存这些状态 |
| 闭包一定按引用捕获 | 按**实际用法**推断：只读借、要改写可变借、要拿走才 move |
| `move` 一定复制数据 | `move` 是"把捕获方式改成拿所有权"，对 `String` 这类类型依然是 move，不是深拷贝 |
| 同一个闭包不能调用两次 | 只要捕获方式不需要拿走（`Fn` / `FnMut`），就能反复调用 |

**在 tmdb-organizer 里**：第 18 周的排序（按 season、episode 排好文件顺序）和后面第 14 周 `--dry-run` 里"筛选出这一季的文件再打印计划"，都是闭包加 `sort_by` / `filter` 的组合——闭包在这里的价值是把"排序规则""筛选条件"写成贴着使用处的一小段代码，而不是散落成一堆具名函数。

### D19 · 迭代器与惰性求值

📖 [书 §13.2](https://kaisery.github.io/trpl-zh-cn/ch13-02-iterators.html)

**迭代器是状态机，不是集合。** `files.iter()` 造出来的只是个"当前位置"的游标对象，每次 `next()` 返回 `Option<&AnimeFile>`，`None` 表示到头了。`for f in &files` 就是编译器把循环翻译成"反复调 `next()`"的糖——所以任何 `Iterator` 都能直接 `for`，反过来 `for` 也没什么神秘。

**惰性：适配器只是"再包一层"。** `filter`、`map`、`take`、`skip` 这类**迭代器适配器**被调用时一个元素都不处理，只是返回一个新的迭代器值；真正驱动一切的是**消费者**（`count`、`sum`、`collect`、`for`、`for_each`）。判断方法看返回值：返回迭代器的是适配器，返回 `usize` / `Vec` / `String` 的是消费者。用了适配器却记不住写消费者，电脑不会有任何反应——这本来就是设计。

**惰性带来两个真实好处。** 一是**不白干活**：`filter(...).take(1)` 找到第一个满足的就停，不会扫完整个目录；二是**没有中间集合**：`filter(...).map(...).collect()` 不会先生成"过滤后的临时 `Vec`"，每个元素直着流过整条链，这正是零成本抽象能成立的原因之一。

```rust
#[derive(Debug)]
struct AnimeFile {
    name: String,
    season: u32,
    episode: u32,
}

fn main() {
    let files = vec![
        AnimeFile { name: String::from("Bleach"), season: 1, episode: 1 },
        AnimeFile { name: String::from("Bleach"), season: 1, episode: 2 },
        AnimeFile { name: String::from("Bleach"), season: 2, episode: 1 },
    ];

    // filter 本身不干活，靠 count 这类消费者驱动
    let season1 = files.iter().filter(|f| f.season == 1);
    println!("第 1 季有几集：{}", season1.count()); // 2

    // 惰性：这条链没有消费者，闭包体根本不会执行
    let _dangling = files.iter().filter(|f| {
        println!("这行不会打印：{}", f.episode);
        f.season == 9
    });
    println!("链条没被消费，上面那行不会执行");
}
```

⚠️ 三个坑：第一，把适配器孤零零写成一行语句，得到警告 `unused `Filter` that must be used`（提示 `iterators are lazy and do nothing unless consumed`）——它不是错误，但说明你漏掉了消费者；第二，`filter` 的闭包收到的是 `&&AnimeFile`（因为 `iter()` 已经给出 `&T`），写 `|f| f.season == 1` 能过是靠自动解引用，要显式写类型就得写 `&&AnimeFile`；第三，在适配器闭包里往原 `Vec` 里 `push` 会撞 E0502 `cannot borrow `files` as mutable because it is also borrowed as immutable`——遍历的不可变借用还在生效，链式写法并不豁免借用规则。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `filter` / `map` 会马上处理数据 | 惰性，只返回新的迭代器；必须由消费者驱动才执行 |
| 链式调用会生成一堆中间 `Vec` | 不会。元素逐个流过整条链，中途不建临时集合 |
| 链式一定比 `for` 慢 | 通常生成同样的机器码；`take`、`find` 这类还能提前停下，反而更快 |
| 迭代器用完一次就废了 | 取决于类型：`iter()` 可以再取；`into_iter()` 消费掉 `Vec` 后原变量失效 |

**在 tmdb-organizer 里**：第 8 周扫目录后会先 `files.iter().filter(...)` 筛出要处理的季，再交给后面的重命名计划生成；"只统计数量""只看前几个"这类需求全都靠消费者（`count` / `take` / `any`）来落地，而惰性保证了我们**只在真的要结果时才开始工作**。

### D20 · 迭代器适配器 `map` / `collect`

📖 [书 §13.3](https://kaisery.github.io/trpl-zh-cn/ch13-03-improving-our-io-project.html)

**`map` 是"逐个变形"，严格一对一。** 输入 N 个元素，输出还是 N 个（类型可以不同），它既不会删除也不会新增。闭包拿到的就是迭代器的元素类型：`iter()` 之后是 `&T`，所以 `|f| format!(...)` 里的 `f` 是 `&AnimeFile`。想按条件丢弃用 `filter`，想一对一变形用 `map`，想压成一个值用 `fold` / `sum`。

**`collect` 是"把迭代器倒进容器"，靠目标类型决定倒进哪儿。** 它本身不知道要 `Vec` 还是 `HashMap`，所以必须能从上下文推断出目标类型——最规范的做法就是在 `let` 左边标清楚：`let labels: Vec<String> = ...collect();`。`Vec`、`String`、`HashMap`、甚至 `Result` 都能收，因为 `collect` 是泛型方法，具体靠目标的 `FromIterator` 实现决定。

| 目标类型 | 元素需要是 | 结果 |
| ---- | ---- | ---- |
| `Vec<String>` | `String` | 收集成列表 |
| `String` | `char` / `&str` / `String` | 拼成一个字符串 |
| `HashMap<u32, String>` | `(u32, String)` 元组 | 收成映射，重复 key 后到的赢 |

**`collect` 收成 `HashMap` 为什么可行。** 因为标准库给 `FromIterator<(K, V)>` 写了实现：迭代器元素的"第一个分量当 key、第二个当 value"。所以 `map(|f| (f.episode, f.name.clone()))` 造出的一对对元组，一个 `collect` 就变成了映射表。这也是"用类型表达意图"的又一次体现——**你只要把元素摆成对，容器就自己组装好了**。

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct AnimeFile {
    name: String,
    season: u32,
    episode: u32,
}

fn main() {
    let files = vec![
        AnimeFile { name: String::from("Bleach"), season: 1, episode: 1 },
        AnimeFile { name: String::from("Bleach"), season: 1, episode: 2 },
    ];

    // map 逐个转换，collect 把结果收成集合
    let labels: Vec<String> = files
        .iter()
        .map(|f| format!("S{:02}E{:02}", f.season, f.episode))
        .collect();
    println!("{:?}", labels); // ["S01E01", "S01E02"]

    // 收成别的类型也行，目标类型由左边标注决定
    let joined: String = labels.join(", ");
    println!("{joined}"); // S01E01, S01E02

    // 元组形状的元素可以直接收成 HashMap：集号 → 系列名
    let by_episode: HashMap<u32, String> = files
        .iter()
        .map(|f| (f.episode, f.name.clone()))
        .collect();
    println!("{:?}", by_episode.get(&1)); // Some("Bleach")
}
```

⚠️ 三个坑：第一，漏写目标类型，报 `type annotations needed`（E0283），`-->` 指向 `collect()` 调用处，修法是在 `let` 上标 `Vec<String>` 或写 `collect::<Vec<String>>()`；第二，`map` 产出的元素和目标容器不匹配，报 `a value of type `Vec<String>` cannot be built from an iterator over elements of type `u32`（E0277），它直接把"元素其实是 `u32`"写出来了，看这句就能定位；第三，`.name.clone()` 忘写就是"想从 `&AnimeFile` 里搬走字段"，报 `cannot move out of `f.name` which is behind a shared reference`（E0507）——`map` 的闭包只借到了元素，搬不走它的字段。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `collect` 只能收成 `Vec` | 目标由 `FromIterator` 决定：`Vec`、`String`、`HashMap`、`Result` 都行 |
| `map` 里可以直接删元素 | 不行，`map` 严格一对一；要删用 `filter` 或 `filter_map` |
| 链式结尾可以不写 `collect` | 不写就没有消费者，链上的闭包一个都不会执行 |
| 收成 `HashMap` 时重复 key 会报错 | 不报错，后到的值覆盖先到的；想累加得改用 `for` + `entry` |

**在 tmdb-organizer 里**：第 14 周生成重命名计划时，`files.iter().map(|f| RenamePlan { from: ..., to: ... }).collect::<Vec<_>>()` 是核心一步；而把"旧文件名 → 标签"做成 `HashMap` 的用法会在打印 dry-run 对照表时用到——`collect` 收 `HashMap` 让这份对照表三行就能建好。

### D21 · 复盘：把 `for` 循环改写成链式调用

📖 [书 §13.2](https://kaisery.github.io/trpl-zh-cn/ch13-02-iterators.html)、[书 §13.3](https://kaisery.github.io/trpl-zh-cn/ch13-03-improving-our-io-project.html)

**不是所有 `for` 都该改。** 判据是**这个循环在干什么**：如果它做的事就是"过滤 + 变形 + 收集/计数"，链式写法更短、意图更集中；如果循环体里有多分支、要 `break`、要写文件、要更新好几个外部状态，那 `for` 反而更清楚。链式的优势是把意图压在一行里读（滤波条件、变形规则、收集目标一目了然），劣势是没法在中间打断点看每一步。

**改写的四步机械映射。** ①`for f in &files {` → `files.iter()`；②循环里的 `if cond { ... }` → `.filter(|f| cond)`；③`push(expr)` 里的 `expr` → `.map(|f| expr)`；④`push` 本身 → `.collect()`。四步对应四样东西，照这个映射改写基本不会出错；剩下的 `let mut out = vec![];` 也可以直接删掉。

| 命令式 | 链式 |
| ---- | ---- |
| `for f in &files {` | `files.iter()` |
| `if f.season == 1 {` | `.filter(\|f\| f.season == 1)` |
| `out.push(format!(...));` | `.map(\|f\| format!(...))` |
| 循环结束 | `.collect()` |

**性能上不必纠结，但副作用要挪出去。** 链式版本编译后通常和手写循环生成一样的机器码，所以选择依据是**可读性**而不是速度。唯一要注意的是别为了"一行搞定"把副作用塞进 `map`——`map` 表达的是"变形"，放进去的写文件、改全局状态会在别人随手加一个 `collect` 时被意外执行，这是链式写法里最容易埋的坑。

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct AnimeFile {
    name: String,
    season: u32,
    episode: u32,
}

fn main() {
    let files = vec![
        AnimeFile { name: String::from("Bleach"), season: 1, episode: 1 },
        AnimeFile { name: String::from("Bleach"), season: 1, episode: 2 },
        AnimeFile { name: String::from("Naruto"), season: 1, episode: 1 },
    ];

    // 命令式写法：手动 push
    let mut manual: Vec<String> = vec![];
    for f in &files {
        if f.season == 1 {
            manual.push(format!("S{:02}E{:02}", f.season, f.episode));
        }
    }

    // 链式写法：filter + map + collect，与上面完全等价
    let chained: Vec<String> = files
        .iter()
        .filter(|f| f.season == 1)
        .map(|f| format!("S{:02}E{:02}", f.season, f.episode))
        .collect();

    println!("{:?}", manual == chained); // true

    // 要做"按系列计数"这种累加，还是 for + entry 更直白
    let mut report: HashMap<&str, usize> = HashMap::new();
    for f in &files {
        *report.entry(f.name.as_str()).or_insert(0) += 1;
    }
    println!("Bleach 的条目数：{}", report.get("Bleach").unwrap_or(&0)); // 2
    println!("Naruto 的条目数：{}", report.get("Naruto").unwrap_or(&0)); // 1
}
```

⚠️ 三个坑：第一，聚合目标类型要和元素类型一致：`let total: usize = files.iter().map(|f| f.episode).sum();` 报 `a value of type `usize` cannot be made by summing an iterator over elements of type `u32`（E0277），要么标 `u32`，要么先 `as usize`；第二，`collect` 收 `HashMap` 时**相同 key 只保留最后一个**，想做计数必须改成 `for` + `entry(...).or_insert(0) += 1`（或者 `fold`），否则数据会静默丢失；第三，一边链式遍历 `files`、一边在闭包里改另一个借用了 `files` 的容器（比如 `HashMap<&str, _>` 存了 `f.name` 的借用），会报 E0502——**借用规则在链式写法里一点都没放松**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 链式一定比 `for` 快 | 通常生成相同的机器码；快慢取决于算法，不是写法 |
| 链式写起来更高级，所以一律该用 | 有 `break`、多分支、写文件时 `for` 更清楚；选择依据是可读性 |
| `filter` + `map` 会遍历两遍 | 一趟流过：每个元素依次经过 `filter` 判断和 `map` 变形，绝不重新开始 |
| 在 `map` 里做副作用也没关系 | `map` 是"变形"的语义，副作用会被后来的消费者意外触发，应挪到 `for` / `for_each` |

**在 tmdb-organizer 里**：第 14 周生成 dry-run 报告就是这一天要练的东西——"筛出目标季的文件 → 生成旧名到新名的标签 → 收集成列表 → 打印"，前半段正好是 `filter` + `map` + `collect`；而"各系列的条目计数"这种带累加统计的部分，仍是 `for` + `entry` 更直白，两种写法在同一个文件里并存很正常。

---

<a id="w4"></a>

## 第 4 周：错误处理 Option、Result、`?`、自定义错误

| 天   | 学 6 min                                  | 写 10 min                                                    | 验收 / commit           |
| ---- | ----------------------------------------- | ------------------------------------------------------------ | ----------------------- |
| D22  | `panic!` 与 `unwrap` 的风险（[书 §9.1](https://kaisery.github.io/trpl-zh-cn/ch09-01-unrecoverable-errors-with-panic.html)） | 找一个 `unwrap()`，改成 `match` 处理错误                     | 不 panic，`day22`       |
| D23  | `Result<T, E>` 与 `match`（[书 §9.2](https://kaisery.github.io/trpl-zh-cn/ch09-02-recoverable-errors-with-result.html)） | `fn parse_episode(s: &str) -> Result<u32, ParseIntError>`    | 打印 Ok/Err，`day23`    |
| D24  | `?` 运算符与错误传播（[书 §9.2](https://kaisery.github.io/trpl-zh-cn/ch09-02-recoverable-errors-with-result.html)） | `parse_filename -> Result<(u32,u32), String>`                | 用 `?` 传播，`day24`    |
| D25  | `Option` 与 `Result` 互转（[书 §9.3](https://kaisery.github.io/trpl-zh-cn/ch09-03-to-panic-or-not-to-panic.html)） | `fn season_from(name: &str) -> Option<u32>`                  | 打印 None/Some，`day25` |
| D26  | 自定义错误类型与 `Display`（[std `Error`](https://rustwiki.org/zh-CN/std/error/trait.Error.html)） | `enum ParseError { MissingSeason, MissingEpisode, InvalidNumber }` + `Display` | 能打印错误，`day26`     |
| D27  | `From` 转换与错误重构（[书 §9.2](https://kaisery.github.io/trpl-zh-cn/ch09-02-recoverable-errors-with-result.html)） | `parse_filename` 返回 `Result<(u32,u32), ParseError>`        | 编译通过，`day27`       |
| D28  | 复盘：错误路径的测试                            | 给 `parse_filename` 写 3 个 `#[test]` 或手动用例             | `cargo test`，`day28` |

### D22 · `panic!` 与 `unwrap` 的风险

📖 [书 §9.1](https://kaisery.github.io/trpl-zh-cn/ch09-01-unrecoverable-errors-with-panic.html)

**panic 是"中止"，不是"报错"。** `panic!` 会打印信息、展开栈、让整个线程退出，中间没有"交给调用方补救"这一步。所以它只该用在**程序内部不变式被破坏**的场合——写死的常量算错了、索引越界了。Rust 故意把错误分成"可恢复（`Result`）"和"不可恢复（panic）"两类，就是逼你在写下这行代码时先回答：**这件事失败了，是有人能处理，还是只能崩？**

**`unwrap` 是"把可能失败偷偷改成必须成功"。** `Option::unwrap` / `Result::unwrap` 的语义是"我赌这里是 `Some`/`Ok`，否则崩"。它不总是坏习惯——原型、测试、一次性脚本里很省事；问题在于**它把失败从类型签名里抹掉了**：函数不再告诉调用方"我可能失败"，上层也就无从防御。批量整理剧集时，一个坏文件名上的 `unwrap` 会让整批处理直接中止，前面已完成的全部白做。

**替代方案按"失败后该继续还是该停"来选。** 能跳过的（个别无法识别的文件）用 `match` / `if let` 兜住，或直接返回 `Option`/`Result` 把决定权交上去；真的不该发生的（内部不变式）才留 `unwrap`，而且建议换成 `expect("...")`，把上下文写进 panic 信息，排查时省事得多。换句话说：**`unwrap` 不是不能用，而是不能用来掩盖"这里其实可能失败"这个事实。**

```rust
// panic! / unwrap 演示：解析失败的两种态度
fn parse_episode(s: &str) -> Option<u32> {
    s.parse::<u32>().ok() // parse 失败返回 None，而不是崩掉
}

fn main() {
    let raw = "abc";

    // 危险写法（注释掉，取消注释会 panic）：
    // let broken: u32 = raw.parse().unwrap();
    // 运行期报 thread 'main' panicked at ... called `Result::unwrap()` on an `Err` value

    // 安全写法：把"没有值"交回调用方决定怎么处理
    match parse_episode(raw) {
        Some(n) => println!("解析成功：第 {n} 集"),
        None => println!("解析失败：{raw} 不是数字，跳过"),
    }

    println!("{:?}", parse_episode("366")); // Some(366)
}
```

⚠️ 三个坑：第一，`unwrap` 崩掉的运行期信息是 **called `Result::unwrap()` on an `Err` value** 或 **called `Option::unwrap()` on a `None` value**，`-->` 会直接指向 `unwrap` 那一行——这是**运行期**爆炸，编译器不会提前拦你；第二，把 `Option<u32>` 直接当 `u32` 用，编译期就报 **mismatched types: expected `u32`, found `Option<u32>`**（E0308），编译器还会附上 **consider using `Option::expect` to unwrap the `Option<u32>` value**——可那个建议仍然是 panic，只是换了个写法；第三，`panic!` 在 `main` 里会让进程以非零码退出，`cargo run` 显示 **process didn't exit successfully**，脚本里这等于"整件事失败"，而不是"这一个文件处理失败"。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `unwrap` 是"取出值"的安全写法 | 它是"失败就崩"的赌注，在 `None` / `Err` 上调用直接 panic |
| panic 能被捕获回来接着跑 | 默认会展开栈并结束线程，不能当普通错误处理用 |
| 原型里随便 `unwrap`，以后再改 | 签名一旦是 `-> u32` 而非 `-> Option<u32>`，上层就无法防御，改动会波及一片 |
| `expect` 比 `unwrap` 更安全 | 行为一样会 panic，只是多了一句上下文；安全性没变 |

**在 tmdb-organizer 里**：`parse_filename` 从一开始就该返回 `Option`/`Result`，而不是在内部 `unwrap`——第 8 周 `scan_dir` 扫到 `sample.mkv` 时，我们要的是"跳过它接着扫"，而不是"整个工具崩掉"。这条原则后面每一周都用到。

### D23 · `Result<T, E>` 与 `match`

📖 [书 §9.2](https://kaisery.github.io/trpl-zh-cn/ch09-02-recoverable-errors-with-result.html)

**`Result` 把"失败的原因"也编进类型。** `Option` 只说"有/没有"，`Result<T, E>` 进一步区分"成功得到 `T`"和"失败得到 `E`"——`E` 就是**失败信息本身**。为什么不用 `bool`？因为 `bool` 只回答"成没成"，把"为什么没成"丢进日志或全局变量，调用方根本拿不到。`Result` 让错误变成**一个普通的、必须被取用的值**，编译器盯着你处理它。

**`ParseIntError` 为什么是类型而不是字符串。** `str::parse` 失败时给你 `std::num::ParseIntError`——一个**结构化错误类型**，而不是一句 "parse failed"。结构化意味着调用方能 `match`、能区分不同原因，而不必去比对字符串内容。代价是打印成人话要 `.to_string()`，好处是错误处理不靠字符串匹配，改文案不会顺手改坏逻辑。

| 类型 | 表达什么 | 失败时给你什么 |
| ---- | ---- | ---- |
| `bool` | 成 / 没成 | 什么都没有 |
| `Option<T>` | 有值 / 没值 | 什么都没有（`None`） |
| `Result<T, E>` | 成功值 `T` / 失败值 `E` | 失败的具体原因 |

**`match` 是处理 `Result` 最完整的写法。** 两个分支都写、都能拿到里面那层值。它的意义在于：只要 `Err` 没被处理，编译器就报错——**错误处理不再是"记得写"，而是"不写就编不过"**。这也是第 4 周反复要建立的习惯：面对 `Result` 先想"两种结果分别怎么办"，再动手写。

```rust
// Result<T, E> 用 match 明确区分成功与失败
fn parse_episode(s: &str) -> Result<u32, std::num::ParseIntError> {
    s.parse::<u32>() // 成功是 Ok(数字)，失败是 Err(ParseIntError)
}

fn main() {
    for raw in ["1", "366", "abc"] {
        match parse_episode(raw) {
            Ok(n) => println!("{raw} -> Ok({n})"),
            Err(e) => println!("{raw} -> Err({e})"),
        }
    }
}
```

⚠️ 三个坑：第一，`println!("{r}")` 打印 `Result` 报 **`Result<u32, ParseIntError>` doesn't implement `std::fmt::Display`**（E0277）——`Result` 没有 `Display`，用 `{:?}` 或先把里层取出来；第二，只写 `Ok` 分支，报 **non-exhaustive patterns: `Err(_)` not covered**（E0004），`Result` 和 D7 的枚举是同一套穷尽性检查；第三，`Ok` 分支返回 `u32`、`Err` 分支只 `println!`（返回 `()`），报 **`match` arms have incompatible types**（E0308）——`match` 是表达式，所有分支必须同类型，错误分支里想"顺手打印一下"也得显式返回一个同类型的值。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `Result` 就是加强版 `bool` | `Result` 携带失败原因 `E`，`bool` 什么原因都不带 |
| `Ok` 和 `Err` 可以只处理一个 | 穷尽性检查会拦住你，两个变体都得覆盖 |
| `Err` 里的值不用管，丢掉就行 | 它是普通值，`match` 出来就归你处置；想保留原因就要存进自己的错误类型（D26） |
| `match` 的分支可以类型不同 | 不行，`match` 是表达式，所有分支必须收敛到同一类型 |

**在 tmdb-organizer 里**：`fn parse_episode(s: &str) -> Result<u32, ParseIntError>` 会是 `parse_filename` 的第一步——把 `"366"` 这样的片段变成数字，失败时把 `ParseIntError` 原样交给上层。**用 `Result` 而不是 `Option` 的关键理由是：解析失败时我们要能说出"哪个字段、什么原因"，而不是默默跳过。**

### D24 · `?` 运算符与错误传播

📖 [书 §9.2](https://kaisery.github.io/trpl-zh-cn/ch09-02-recoverable-errors-with-result.html)

**`?` 是 `match` 的语法糖，省的是错误分支的样板。** `let n = foo()?;` 大致展开成 `let n = match foo() { Ok(v) => v, Err(e) => return Err(From::from(e)) };`。它做两件事：成功取值继续，失败**提前返回**把错误交给上层。于是函数主干能写得像"永远不会失败"一样，失败这件事只在签名 `-> Result<...>` 里体现。

**关键细节在 `From::from`。** 提前返回前会先 `e.into()`，把当前错误类型**转换成函数自己声明的错误类型**。这正是 `?` 同时表达"向上传播"和"类型转换"两件事的原因：底层抛 `ParseIntError`，上层要 `String`，中间的桥就是 `From`。也正因为它要"提前 return"，`?` 只能出现在返回 `Result`/`Option` 的函数里。

**什么时候该 `?`、什么时候该 `match`。** `?` 适合"我不处理，交给上层"，让错误沿调用链上走；如果这一层需要**补偿**（用默认值、跳过、换策略），就得 `match`/`unwrap_or` 在这里把错误吃掉。判据只有一句：**这个错误的最终处理者，是不是当前这一层。**

```rust
// ? 运算符：出错就把错误"原样"往上抛，不再一层层手写 match
fn parse_number(s: &str) -> Result<u32, String> {
    s.parse::<u32>().map_err(|e| format!("不是数字：{e}"))
}

fn parse_filename(name: &str) -> Result<(u32, u32), String> {
    let s = name.find('S').ok_or_else(|| format!("缺少 S：{name}"))?;
    let e = name.find('E').ok_or_else(|| format!("缺少 E：{name}"))?;
    let season = parse_number(&name[s + 1..e])?; // 出错就提前 return Err
    let tail = &name[e + 1..];
    let cut = tail.find('.').unwrap_or(tail.len()); // Option 用 unwrap_or 给个默认值
    let episode = parse_number(&tail[..cut])?;
    Ok((season, episode))
}

fn main() {
    for name in ["Bleach.S01E366.mkv", "Bleach.S01.mkv", "Bleach.SxxE01.mkv"] {
        match parse_filename(name) {
            Ok((s, e)) => println!("{name} -> S{s:02}E{e:02}"),
            Err(err) => println!("{name} -> 错误：{err}"),
        }
    }
}
```

⚠️ 三个坑：第一，在返回 `()` 的普通 `main` 里用 `?`，报 **the `?` operator can only be used in a function that returns `Result` or `Option` (or another type that implements `FromResidual`)**（E0277）——想图省事就得把签名改成 `fn main() -> Result<(), Box<dyn std::error::Error>>`（第 15 周再说）；第二，把 `Option` 的结果用 `?` 传进返回 `Result` 的函数，报 **the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`**（E0277），编译器直接给出修法 **use `.ok_or(...)?` to provide an error compatible with `Result<(u32, u32), String>`**；第三，错误类型对不上时报 **`?` couldn't convert the error to `String`**（E0277）——`ParseIntError` 不会自动变 `String`，要么 `.map_err(|e| e.to_string())`，要么给自定义错误实现 `From`（D27）。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `?` 会吞掉错误 | 它把错误**往上抛**，一层层直到有人 `match` 处理或 `main` 兜底 |
| `?` 是"忽略错误继续跑" | 它会让当前函数**立刻返回**，后面的代码根本不会执行 |
| `Option` 和 `Result` 的 `?` 可以混用 | 不行，`?` 的返回类型必须和函数签名一致，跨类型要 `.ok_or(...)?` |
| 用了 `?` 就不必懂 `match` | `?` 就是 `match` 的糖；不懂 `match`，出错时根本不知道该在哪层修 |

**在 tmdb-organizer 里**：`fn parse_filename(name: &str) -> Result<(u32, u32), String>` 里会连着四次 `?`——定位 `S`、定位 `E`、解析季号、解析集号，每一步失败都立刻把原因抛给调用方。第 8 周 `scan_dir` 调用它时，用 `filter_map(|p| parse_filename(...).ok())` 就能把成功解析的文件留下、其余安静跳过，**一行代码同时表达了"继续"和"为什么继续"**。

### D25 · `Option` 与 `Result` 互转

📖 [书 §9.3](https://kaisery.github.io/trpl-zh-cn/ch09-03-to-panic-or-not-to-panic.html)

**两者差在"要不要解释原因"。** `Option` 只说"可能没有"，`Result` 还说"为什么没有"。所以互转的实质是：`Option → Result` 要**补一个错误值**，用 `ok_or`/`ok_or_else`；`Result → Option` 要**丢掉错误值**，用 `.ok()`。一补一丢，信息量在两者之间流动。

**`ok_or` 与 `ok_or_else` 的区别是"急不急"。** `ok_or(x)` 的参数是**已经算好的值**，无论成功失败都会先构造出来；`ok_or_else(|| ...)` 传闭包，只在真要失败时才调用它。所以带 `format!` 的错误优先用 `ok_or_else`——在高频成功的路径上，前者会白白拼一遍字符串。

**`?` 在两边行为一致，但不能跨类型。** 在 `Option` 上用 `?`，`None` 就返回 `None`；在 `Result` 上用 `?`，`Err` 就返回 `Err`。在一个返回 `Result` 的函数里对 `Option` 用 `?` 直接编不过——这其实是好约束：**"这个函数失败时能不能说出原因"，必须在签名里一次讲清**，不能含混。

| 方向 | 方法 | 需要给出什么 |
| ---- | ---- | ---- |
| `Option → Result` | `ok_or(err)` / `ok_or_else(闭包)` | 一个错误值，或一个"需要时才造它"的闭包 |
| `Result → Option` | `.ok()` | 丢掉错误，只留"成没成" |
| 翻转嵌套 | `.transpose()` | 把 `Option<Result<T, E>>` 翻成 `Result<Option<T>, E>` |

```rust
// Option 与 Result 互转：两种"可能失败"的表现形式
fn season_from(name: &str) -> Option<u32> {
    // 形如 Bleach.S01E01.mkv
    let s = name.find('S')?; // ? 用在 Option 上：None 直接返回
    let e = name.find('E')?;
    if e <= s + 1 {
        return None;
    }
    name[s + 1..e].parse::<u32>().ok() // Result -> Option：失败变 None
}

fn main() {
    println!("{:?}", season_from("Bleach.S01E01.mkv")); // Some(1)
    println!("{:?}", season_from("Bleach.mkv"));       // None

    // Option -> Result：给"没有"补一条错误信息
    let raw = "Bleach.mkv";
    let r: Result<u32, String> =
        season_from(raw).ok_or_else(|| format!("无法从 {raw} 读出季号"));
    println!("{:?}", r); // Err("无法从 Bleach.mkv 读出季号")

    // Result -> Option：丢掉错误细节，只关心成没成
    let n: Result<u32, std::num::ParseIntError> = "366".parse();
    println!("{:?}", n.ok()); // Some(366)
}
```

⚠️ 三个坑：第一，在返回 `Result` 的函数里对 `Option` 用 `?`，报 **the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`**（E0277）；第二，`ok_or(format!(...))` 在热路径上是**每次都拼字符串**，改成 `ok_or_else(|| format!(...))` 才是"失败时才拼"；第三，`.ok()` 会**静默丢弃错误信息**——一旦从 `Result` 变成 `Option`，"为什么失败"就再也说不出了，排查问题的地方别顺手 `.ok()`。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `None` 和 `Err` 是一回事 | `None` = "没有值"，`Err` = "失败了，原因是 e"，信息量不同 |
| `ok_or` 和 `ok_or_else` 只是写法偏好 | 前者总是先构造错误值，后者惰性；带 `format!` 时性能差别看得见 |
| `.ok()` 只是类型转换，没有代价 | 它丢掉了错误原因，是一种**有损**转换，排查时可能再也查不到 |
| `?` 会自动把 `Option` 转成 `Result` | 不会，必须显式 `.ok_or(...)?`，编译器只负责提示 |

**在 tmdb-organizer 里**：`fn season_from(name: &str) -> Option<u32>` 这种"只关心能不能读出季号"的辅助函数用 `Option` 最自然；一旦要把结果交回给需要报错的流程（比如给用户打印"哪个文件为什么失败"），就在边界处 `ok_or_else` 升格成 `Result`。**`Option` 用在内部快速判断，`Result` 用在最终对用户交代**，这条分工第 8 周和第 14 周都会体现。

### D26 · 自定义错误类型与 `Display`

📖 [std `Error`](https://rustwiki.org/zh-CN/std/error/trait.Error.html)

**为什么不能一直拿 `String` 当错误。** `String` 省事，但调用方**没法可靠区分**——想知道是"缺季号"还是"数字非法"，只能去比对字符串内容，脆弱且低级。自定义 `enum ParseError` 把失败**收敛成有限几种**：调用方能 `match` 出精确分支，日后新增一种错误时，所有没处理它的 `match` 会一起编译报错，等于免费的影响面清单。

**`Display` 与 `Debug` 分工不同。** `Debug`（`{:?}`）给**开发者**看：输出类型和字段原貌，适合日志、测试断言；`Display`（`{}`）给**用户日志**看：写成人话。所以 `#[derive(Debug)]` 一行就有，`Display` 必须手写 `impl`。注意在 `fmt` 里写 `write!(f, "...")` 而不是 `println!`——你拿到的是 `Formatter`，职责只是"往缓冲区写一段"，打印与否由调用方决定。

**`std::error::Error` 是"我是错误"的认证书。** 它定义为 `pub trait Error: Debug + Display`，所以**必须先有 `Debug` 和 `Display` 才能 `impl` 它**。实现之后，错误就能被 `?` 转换、能装箱成 `Box<dyn Error>`、能被各种库接受——第 15 周引入 `anyhow` 时吃的就是这口饭。这里不用写任何方法，一个空 `impl` 就够，因为默认方法已经够用。

```rust
use std::fmt;

// 用枚举把"文件名为什么解析不了"这件事表达出来
#[derive(Debug)]
enum ParseError {
    MissingSeason,
    MissingEpisode,
    InvalidNumber,
}

// 实现 Display，就能用 {} 打印出给人看的错误信息
impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let msg = match self {
            ParseError::MissingSeason => "缺少季号（S）",
            ParseError::MissingEpisode => "缺少集号（E）",
            ParseError::InvalidNumber => "集号不是合法数字",
        };
        write!(f, "{msg}")
    }
}

// 实现标准库的 Error：自动获得"能被 ? 转换、能装箱"这些能力
impl std::error::Error for ParseError {}

fn main() {
    let e = ParseError::MissingEpisode;
    println!("{}", e);   // Display：缺少集号（E）
    println!("{:?}", e); // Debug：MissingEpisode
}
```

⚠️ 三个坑：第一，只 `impl std::error::Error` 却没写 `Display`，报 **`ParseError` doesn't implement `std::fmt::Display`**（E0277），并明确写着 **required by a bound in `std::error::Error`**（`pub trait Error: Debug + Display`）——顺序是先把 `Display` 补上；第二，忘了 `impl fmt::Display` 就想 `println!("{e}")`（哪怕 `Debug` 已派生），同样报 **`ParseError` doesn't implement `std::fmt::Display`**——`{}` 只认 `Display`，`{:?}` 才走 `Debug`；第三，`impl Display` 前忘了 `use std::fmt;` 却直接写 `impl Display for ...`，报 **cannot find trait `Display` in this scope**（E0405），编译器会提示 **consider importing this trait**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 自定义错误必须写一堆 trait 方法 | `impl Display` 写一个 `fmt`，再空 `impl std::error::Error` 即可 |
| `Error` 可以单独实现 | 它要求 `Debug + Display` 先就位，否则 `impl` 处就报 E0277 |
| `{}` 打不出来就改用 `{:?}` 凑合 | `{:?}` 是给开发者看的类型原貌，用户日志该用 `Display` 写人话 |
| `write!` 和 `println!` 只差一个换行 | `write!` 需要传入输出目标 `f`，不写会报参数个数不匹配 |

**在 tmdb-organizer 里**：`ParseError` 会放进 `parser`（第 7 周拆分），让 `parse_filename` 的返回类型从 `Result<_, String>` 升级成 `Result<_, ParseError>`。这么做之后，第 9 周 CLI 层可以用 `eprintln!("跳过 {path}：{err}")` 把 `Display` 内容直接讲给用户听，而测试里用 `{:?}` 断言具体是哪个变体。

### D27 · `From` 转换与错误重构

📖 [书 §9.2](https://kaisery.github.io/trpl-zh-cn/ch09-02-recoverable-errors-with-result.html)

**`?` 的自动转换，靠的就是 `From`。** `?` 在提前返回前会执行 `From::from(e)`。当你给 `ParseError` 实现了 `From<ParseIntError>`，`tail[..cut].parse()?` 这行就同时完成了"解析"和"把 `ParseIntError` 包成 `ParseError::InvalidNumber`"两件事，代码里一个 `map_err` 都不用写。**这就是 D24 里"错误类型对不上"的正解：与其到处 `.map_err`，不如实现一次 `From`。**

**为什么要"包住"底层错误而不是丢掉。** `ParseError::InvalidNumber(ParseIntError)` 把底层错误**存进变体里**，于是错误类型既是"我这种错误"又是"它能给出细节"：日志里打印 `{e}` 能得到 `invalid digit found in string`，需要时还能对底层做更细的 `match`。等价写法是 `Box<dyn Error>`，但那样就退回到"无法区分种类"，本条链路的取舍是**保留可枚举性**。

**重构方向：让错误类型跟着分层一起长。** 起初用 `String` 最快，一旦有了多层（解析层、计划层、IO 层），就该每层一个错误枚举、在边界上写 `From`，让 `?` 把它们缝起来。这条路线正是第 15 周 `anyhow` 想自动化的东西——**先用标准库手写一遍，才知道 `anyhow` 到底替你省了什么。**

```rust
use std::fmt;
use std::num::ParseIntError;

// 自定义错误类型：把底层错误"包"进来，保留细节
#[derive(Debug)]
enum ParseError {
    MissingSeason,
    MissingEpisode,
    InvalidNumber(ParseIntError),
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParseError::MissingSeason => write!(f, "缺少季号（S）"),
            ParseError::MissingEpisode => write!(f, "缺少集号（E）"),
            ParseError::InvalidNumber(e) => write!(f, "数字非法：{e}"),
        }
    }
}

// From 让 ? 能把 ParseIntError 自动转成 ParseError
impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::InvalidNumber(e)
    }
}

fn parse_filename(name: &str) -> Result<(u32, u32), ParseError> {
    let s = name.find('S').ok_or(ParseError::MissingSeason)?;
    let e = name.find('E').ok_or(ParseError::MissingEpisode)?;
    let season: u32 = name[s + 1..e].parse()?; // 自动 From 转换
    let tail = &name[e + 1..];
    let cut = tail.find('.').unwrap_or(tail.len());
    let episode: u32 = tail[..cut].parse()?;
    Ok((season, episode))
}

fn main() {
    for name in ["Bleach.S01E366.mkv", "Bleach.S01.mkv", "Bleach.episode.mkv"] {
        match parse_filename(name) {
            Ok((s, e)) => println!("{name} -> S{s:02}E{e:02}"),
            Err(err) => println!("{name} -> 错误：{err}"),
        }
    }
}
```

⚠️ 三个坑：第一，忘了 `impl From<ParseIntError> for ParseError`，`parse()?` 立刻报 **`?` couldn't convert the error to `ParseError`**（E0277）——`?` 不会魔法，桥得你自己搭；第二，加了 `InvalidNumber(ParseIntError)` 这个带数据的变体后，`Display` 的 `match` 漏了它，报 **non-exhaustive patterns: `ParseError::InvalidNumber(_)` not covered**（E0004）——带数据的变体在模式里要写成 `InvalidNumber(e)` 或 `InvalidNumber(_)`；第三，把 `From` 写成"自己转自己"（`impl From<ParseIntError> for ParseIntError`）报 **conflicting implementations**（E0119），因为它和标准库里的 `impl<T> From<T> for T` 撞了。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `?` 会自动把任何错误转成任何类型 | 只走 `From` 搭好的桥，没实现 `From` 就报 E0277 |
| `From` 和 `Into` 要各写一遍 | 实现了 `From<A> for B`，`Into<B> for A` 由标准库自动给出 |
| 自定义错误必须丢掉底层细节 | 把底层错误**存进变体**，既保留种类又保留细节 |
| 一开始就该设计完备的错误体系 | 先 `String`、再按层长出枚举、最后用 `anyhow` 收口，随复杂度演进最省事 |

**在 tmdb-organizer 里**：`parse_filename -> Result<(u32, u32), ParseError>` 是这一天的成果——`ParseError` 枚举在 `parser.rs` 里定义，`From<ParseIntError>` 让两个 `.parse()?` 干净得像不会失败。第 11 周 `serde_json` 的错误也会用同样思路接进来，第 15 周则交给 `anyhow` 统一。

### D28 · 复盘：错误路径的测试

📖 [书 §9.2](https://kaisery.github.io/trpl-zh-cn/ch09-02-recoverable-errors-with-result.html)

**错误路径比成功路径更值得测。** 成功路径平时手动跑一次就看见了，错误路径却常年在生产里潜伏——直到遇到一个 `Bleach.SxxE01.mkv`。写测试的价值就是把"我以为会失败的情况"变成**可重复执行**的断言：改了解析逻辑，一个 `cargo test` 就知道有没有把错误路径改坏。

**断言错误要用 `assert_eq!` 比整个 `Result`，而不是只判 `is_err()`。** `assert_eq!(parse_filename("Bleach.S01.mkv"), Err(ParseError::MissingEpisode))` 同时钉死"失败了"和"原因正确"。只写 `assert!(r.is_err())` 太松——把"缺集号"误报成"缺季号"也能通过。前提是 `ParseError` 要 `#[derive(PartialEq, Debug)]`，`assert_eq!` 两样都要。

**手动断言就是 `#[test]` 的平替。** 这里用 `assert_eq!` + `println!` 是为了能在单文件里直接 `rustc` 跑；放进 cargo 项目时，把这几行搬进 `#[cfg(test)] mod tests { ... }` 里的 `#[test] fn ...` 即可（第 7 周正式讲模块与测试）。**验收标准一样：三个用例全过。**

```rust
// 复盘：给错误路径也写上用例（这里用手写断言，等价于 #[test]）
#[derive(Debug, PartialEq)]
enum ParseError {
    MissingSeason,
    MissingEpisode,
    InvalidNumber,
}

fn parse_filename(name: &str) -> Result<(u32, u32), ParseError> {
    let s = name.find('S').ok_or(ParseError::MissingSeason)?;
    let e = name.find('E').ok_or(ParseError::MissingEpisode)?;
    let season: u32 = name[s + 1..e].parse().map_err(|_| ParseError::InvalidNumber)?;
    let tail = &name[e + 1..];
    let cut = tail.find('.').unwrap_or(tail.len());
    let episode: u32 = tail[..cut].parse().map_err(|_| ParseError::InvalidNumber)?;
    Ok((season, episode))
}

fn main() {
    // 正常路径
    assert_eq!(parse_filename("Bleach.S01E366.mkv"), Ok((1, 366)));
    // 错误路径一：缺集号
    assert_eq!(parse_filename("Bleach.S01.mkv"), Err(ParseError::MissingEpisode));
    // 错误路径二：数字非法
    assert_eq!(parse_filename("Bleach.SxxE01.mkv"), Err(ParseError::InvalidNumber));

    println!("3 个用例全部通过"); // 3 个用例全部通过
}
```

⚠️ 三个坑：第一，`ParseError` 没 `#[derive(PartialEq)]` 就 `assert_eq!`，报 **`ParseError` doesn't implement `PartialEq`**（E0277），而且 `assert_eq!` 同时还需要 `Debug`；第二，`#[test]` 函数在普通 `rustc` 编译下**不会执行**（只有 `cargo test` 才跑），本地想验证得手动在 `main` 里调一遍，别以为编译过了就等于测过了；第三，断言写成 `Err(_)` 只验证"确实失败"，把"原因是不是缺集号"这层检查丢了——除非你本来只想验证"它是 `Err`"，否则请写全变体。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 成功用例过了就算测好了 | 错误路径才是回归高发区，要单独给用例 |
| `assert!(r.is_err())` 和 `assert_eq!(r, Err(...))` 等价 | 前者只验证"失败"，后者还验证"失败原因"，更严格 |
| 单文件里写 `#[test]` 会随 `rustc` 一起跑 | `#[test]` 只由 `cargo test` 驱动，`rustc` 编出来也不会执行 |
| 测试要等逻辑全写完再补 | 每加一条错误分支就配一个用例，成本最低、定位最快 |

**在 tmdb-organizer 里**：第 7 周会把这三个用例正式搬进 `tests/parser_test.rs`（或 `parser.rs` 的 `#[cfg(test)] mod tests`）用 `cargo test` 跑；到第 12 周引入 `EpisodeOrder` 解析后，再给 `parse_order` 补一组"未知顺序返回 `Err`"的用例。**先在这里手写、想清断言，第 7 周只是换个存放位置。**

---

<a id="w5"></a>

## 第 5 周：枚举与 match 深入

| 天   | 学 6 min                                   | 写 10 min                                                    | 验收 / commit         |
| ---- | ------------------------------------------ | ------------------------------------------------------------ | --------------------- |
| D29  | `enum` 定义与无数据变体（[书 §6.1](https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html)） | 定义 `enum EpisodeOrder { Tvdb, Dvd, Absolute, Netflix, Crunchyroll }` | 编译通过，`day29`     |
| D30  | 为枚举 `impl` 方法（[书 §6.1](https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html)） | `impl EpisodeOrder { fn label(&self) -> &str }`              | 打印标签，`day30`     |
| D31  | `match` 与穷尽性检查（[书 §6.2](https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html)） | `fn parse_order(s: &str) -> Result<EpisodeOrder, String>`    | 解析 `tvdb`，`day31`  |
| D32  | `match` 绑定值、多模式与 `_`（[书 §6.2](https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html)） | 用 `match` 打印不同 Order 的说明                             | 全部变体覆盖，`day32` |
| D33  | `if let` 与 `let else`（[书 §6.3](https://kaisery.github.io/trpl-zh-cn/ch06-03-if-let.html)） | 定义 `struct Episode { season, episode, title, absolute }`   | 构造一个，`day33`     |
| D34  | 结构体 + 枚举组合建模                       | 定义 `struct RenamePlan { from, to, order }`                 | 构造一个，`day34`     |
| D35  | 复盘：让类型表达业务规则                    | 手动构造 `RenamePlan` 并打印                                 | `cargo run`，`day35`  |

### D29 · `enum` 定义与无数据变体

📖 [书 §6.1](https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html)

**枚举的本质是"取值被限制在一个封闭集合里"。** `EpisodeOrder` 有五种可能，别处拿到的 `EpisodeOrder` 值**必然是这五种之一**，这一点由编译器保证。别的语言常用整数常量（`const TVDB: u8 = 0;`）或字符串表示状态，问题是**没有任何东西挡得住有人传个 `7` 或 `"tvdb2"` 进来**；枚举把这类错误提前到编译期，同时它本身就是一份"到底有哪些状态"的清单。

**无数据变体就是"只有名字、不带信息"。** D7 已见过带数据的变体；当几个状态本身就能说明一切时（`Tvdb` / `Dvd` / `Absolute`），不附数据就够了，构造也简单：`EpisodeOrder::Netflix` 直接就是个值。这是"用类型表达状态"的最小形态——**状态这个词本身成了类型的一部分，而不是注释里的一句话。**

**为什么宁可多定义几个变体，也别用 `String`。** `String` 能表示任何东西，也就意味着**它能表示所有错的东西**；每当需要一个"有限几种情况"的类型，就该想到 `enum`。第 5 周后面几天会依次用 `impl`、`match`、`if let` 把它用起来，但根基在这一天：**先把"有哪几种"写死，再谈怎么处理。**

```rust
// enum 的"无数据变体"：只给几种状态起名字
#[derive(Debug)]
enum EpisodeOrder {
    Tvdb,
    Dvd,
    Absolute,
    Netflix,
    Crunchyroll,
}

fn main() {
    let order = EpisodeOrder::Netflix;
    println!("{:?}", order); // Netflix

    let all = [
        EpisodeOrder::Tvdb,
        EpisodeOrder::Dvd,
        EpisodeOrder::Absolute,
        EpisodeOrder::Netflix,
        EpisodeOrder::Crunchyroll,
    ];
    println!("顺序一共 {} 种", all.len()); // 顺序一共 5 种
}
```

⚠️ 三个坑：第一，把 `EpisodeOrder::Netflix` 写成 `Netflix`（不带前缀）报 **cannot find value `Netflix` in this scope**（E0425）——变体要用 `枚举名::变体名` 全路径引用，除非 `use EpisodeOrder::*`；第二，光定义 `enum` 不 `#[derive(Debug)]` 就 `println!("{:?}", order)`，报 **`EpisodeOrder` doesn't implement `Debug`**（E0277），编译器会提示 **consider annotating `EpisodeOrder` with `#[derive(Debug)]`**；第三，两个变体之间漏了逗号报 **expected one of `(`, `,`, `=`, `{`, or `}`, found `Dvd`**，编译器紧接着提示 **missing `,`**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| enum 只是给整数起名字 | Rust 的 enum 是类型：`EpisodeOrder` 和 `u8` 是两个不同的东西，不能混用 |
| 无数据变体和常量没区别 | 常量可以被任意整数绕过；变体的取值集合由编译器封死 |
| 定义一个 enum 就得立刻写 match | 先定义、再用，可以分天做；没被用到最多是个 dead_code 警告 |
| 变体名必须全局唯一 | 不同 enum 可以有同名变体，靠 `EpisodeOrder::Tvdb` 全路径区分 |

**在 tmdb-organizer 里**：`EpisodeOrder` 是第 12 周要落地的类型——从 TMDB 的 episode group 响应里读出的 `type` 字段，最终要映射成它；第 13 周生成重命名计划、第 14 周 dry-run 打印时，都会拿它当"这套编号按什么顺序"的开关。

### D30 · 为枚举 `impl` 方法

📖 [书 §6.1](https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html)

**方法挂在类型上，枚举和结构体一样。** `impl EpisodeOrder` 里的语法和 D6 学的完全一致：`&self` 只读借用，`self` 拿走所有权。区别只在方法体里通常要 `match self` 才知道"我是哪个变体"。把"变体 → 人话"的映射收进方法，是为了**以后只改一处**：调用方永远写 `o.label()`，不必在十处 `match` 里各抄一份文案。

**返回 `&'static str` 而不是 `String`。** `label` 的返回值来自字符串字面量，字面量活在程序的整个生命周期里，所以能直接借出去、零分配；写成 `String` 就得每次 `format!` / `.to_string()` 造一份新堆数据。这也是 D6 那条判据的复用：**能借就别造**，返回 `&'static str` 正好表达"我指的是写死在程序里的那段文字"。

**`match self` 配合穷尽性检查，是一份可执行的清单。** 方法体里必须列出每个变体，少一个就编译报错。于是给枚举加变体时，编译器会**主动**带你找到所有需要补文案的地方——比靠 grep 搜字符串可靠得多。

```rust
// 为枚举 impl 方法：把"这个名字对应哪套顺序"收进类型自己身上
#[derive(Debug)]
enum EpisodeOrder {
    Tvdb,
    Dvd,
    Absolute,
    Netflix,
    Crunchyroll,
}

impl EpisodeOrder {
    fn label(&self) -> &'static str {
        match self {
            EpisodeOrder::Tvdb => "电视台播出顺序",
            EpisodeOrder::Dvd => "DVD 发售顺序",
            EpisodeOrder::Absolute => "绝对集数",
            EpisodeOrder::Netflix => "Netflix 顺序",
            EpisodeOrder::Crunchyroll => "Crunchyroll 顺序",
        }
    }
}

fn main() {
    for o in [EpisodeOrder::Tvdb, EpisodeOrder::Netflix] {
        println!("{:?} -> {}", o, o.label());
    }
    // Tvdb -> 电视台播出顺序
    // Netflix -> Netflix 顺序
}
```

⚠️ 三个坑：第一，`match` 漏了某个变体，报 **non-exhaustive patterns: `&EpisodeOrder::Dvd` not covered**（E0004）——注意那个 `&`，因为 `&self` 让 `self` 是引用；第二，返回类型写成 `&'static str` 却在分支里 `format!(...)` 造了个 `String`，报 **mismatched types: expected `&str`, found `String`**（E0308）；第三，忘了写 `impl` 块直接 `o.label()`，报 **no method named `label` found for enum `EpisodeOrder`**（E0599）。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 枚举不能有自己的方法 | 和 struct 一样能 `impl`，方法体里用 `match self` 区分变体 |
| 返回 `&str` 不如返回 `String` 保险 | 字面量可以借成 `&'static str`，零分配，是更省的选择 |
| 方法里必须处理所有变体很麻烦 | 这正是好处：加变体时编译器会点出所有漏改的方法 |
| `&self` 和 `self` 在枚举上行为一样 | `&self` 不消耗值，`self` 会 move 掉枚举变量，和 D6 一样 |

**在 tmdb-organizer 里**：`impl EpisodeOrder { fn label(&self) -> &'static str }` 直接给 CLI 打印和 dry-run 报告用——第 9 周 `clap` 解析出的顺序、第 14 周打印的计划表，都靠 `label()` 把枚举翻成人话，而不用在每处 `match` 里重写文案。

### D31 · `match` 与穷尽性检查

📖 [书 §6.2](https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html)

**穷尽性是编译期保证，不是运行时兜底。** `match` 必须覆盖所有可能，漏了就报错。这里的"所有可能"对**枚举**来说是有限且已知的——所以编译器能精确点名漏了哪个变体；对 `&str` 这类**无限**输入不可能穷尽，必须写兜底分支。理解这个区别很关键：**穷尽性能救你的地方，只有类型本身是封闭的（枚举）时。**

**解析函数必须有兜底，业务分支最好没有。** `parse_order("hulu")` 里 `&str` 有无穷多种，`other => Err(...)` 的兜底是**必须**的；但拿到 `EpisodeOrder` 之后再做 `match`，就应该老老实实列全部变体，**不要用 `_`**——因为那时 `_` 会屏蔽"新增变体"的提醒，把编译器主动送上门的检查白白丢掉。同一种语法，两种场景的取舍正好相反。

**穷尽性检查的收益是"改这里，编译器告诉你哪里还要改"。** 给 `EpisodeOrder` 加一个 `Netflix`，所有列全变体的 `match` 立刻集体报错，等于一份自动维护的待办清单。这也是 D7 反复强调"变体少时宁可写全、别写 `_`"的原因。

```rust
// match 与穷尽性：把 TMDB 返回的字符串解析成枚举
#[derive(Debug, PartialEq)]
enum EpisodeOrder {
    Tvdb,
    Dvd,
    Absolute,
    Netflix,
    Crunchyroll,
}

fn parse_order(s: &str) -> Result<EpisodeOrder, String> {
    match s {
        "tvdb" => Ok(EpisodeOrder::Tvdb),
        "dvd" => Ok(EpisodeOrder::Dvd),
        "absolute" => Ok(EpisodeOrder::Absolute),
        "netflix" => Ok(EpisodeOrder::Netflix),
        "crunchyroll" => Ok(EpisodeOrder::Crunchyroll),
        other => Err(format!("未知顺序：{other}")),
    }
}

fn main() {
    for s in ["tvdb", "netflix", "hulu"] {
        match parse_order(s) {
            Ok(o) => println!("{s} -> Ok({o:?})"),
            Err(e) => println!("{s} -> Err({e})"),
        }
    }
    // tvdb -> Ok(Tvdb)
    // netflix -> Ok(Netflix)
    // hulu -> Err(未知顺序：hulu)
}
```

⚠️ 三个坑：第一，`match` 一个 `&str` 却忘了兜底分支，报 **non-exhaustive patterns: `&_` not covered**（E0004），编译器会提示 **ensure that all possible cases are being handled by adding a match arm with a wildcard pattern**；第二，返回 `Result<EpisodeOrder, String>` 时把 `Ok` 忘了包，写成 `"tvdb" => EpisodeOrder::Tvdb`，报 **mismatched types: expected `Result<EpisodeOrder, String>`, found `EpisodeOrder`**（E0308）；第三，`EpisodeOrder` 没派生 `Debug`，调用方用 `{o:?}` 打印就报 **`EpisodeOrder` doesn't implement `Debug`**（E0277）——解析函数往往和打印配套，`#[derive(Debug, PartialEq)]` 一起派生了最省心。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 每个 `match` 都得写 `_` 兜底 | 枚举上的 `match` 写全变体更能吃到穷尽性检查；`_` 只对"开放集合"（如 `&str`）必要 |
| 穷尽性检查是运行期行为 | 纯编译期；编过就说明所有情况都有分支 |
| 解析失败用 `unwrap` 更简单 | 应该返回 `Err`，让上层决定是跳过还是终止（D22-D25 讲的） |
| 加个变体会悄悄破坏旧代码 | 恰恰相反，所有没处理它的 `match` 会立刻报错，反而更安全 |

**在 tmdb-organizer 里**：`fn parse_order(s: &str) -> Result<EpisodeOrder, String>` 是第 12 周 TMDB 响应落地的第一站——`"absolute"` / `"dvd"` / `"tvdb"` 映射成变体，未知字符串返回 `Err` 而不是硬塞成某个默认值；**宁可让用户看到"未知顺序"，也不要静默猜错顺序去改名。**

### D32 · `match` 绑定值、多模式与 `_`

📖 [书 §6.2](https://kaisery.github.io/trpl-zh-cn/ch06-02-match.html)

**模式不只是"相等判断"，它能一次完成判断和拆包。** `EpisodeRef::Tvdb { season, episode }` 既检查"是不是 `Tvdb` 变体"，又把两个字段绑成局部变量。所以 `match` 比一连串 `if ... == ...` 强在：**解构和分支在同一处完成**，不用先判类型、再逐字段取。

**多模式和范围模式是"合并同款分支"。** `season: s @ (1 | 2)` 表示 1 或 2 都走这支，`@` 把命中的那个值绑成 `s`；`n @ 1..=366` 用范围判断并绑定 `n`。它们省掉的是重复的分支代码。**关键顺序规则：分支从上到下、第一个命中就停**，所以具体的、带字面量的分支必须放在宽泛的"纯绑定"分支前面，否则后者会把前面全遮住。

**`_` 是"我不关心这里"，但也可能是"我不关心漏了什么"。** 在结构体模式里 `Tvdb { season, .. }` 用 `..` 忽略其余字段很方便；但如果整个 `match` 用 `_ =>` 兜底枚举，就等于关掉了穷尽性检查。**能把变体写全就写全**，`_` 留给真正开放的情况（D31 已说）。

```rust
// match 绑定值、多模式与范围模式
#[derive(Debug)]
enum EpisodeRef {
    Tvdb { season: u32, episode: u32 },
    Absolute(u32),
    Unknown,
}

fn describe(r: &EpisodeRef) -> String {
    match r {
        // 字面量模式 + 绑定：season 固定为 0 时绑出 episode
        EpisodeRef::Tvdb { season: 0, episode } => format!("特典 E{episode:02}"),
        // 多模式：1 或 2 都命中，s 绑定住命中的那个季号
        EpisodeRef::Tvdb { season: s @ (1 | 2), episode } => format!("正篇 S{s}E{episode:02}"),
        // 兜底绑定：其余季
        EpisodeRef::Tvdb { season, episode } => format!("S{season}E{episode:02}"),
        // 范围模式 + @ 绑定
        EpisodeRef::Absolute(n @ 1..=366) => format!("绝对 #{n:03}"),
        EpisodeRef::Absolute(n) => format!("异常绝对集 #{n}"),
        EpisodeRef::Unknown => String::from("未知"),
    }
}

fn main() {
    println!("{}", describe(&EpisodeRef::Tvdb { season: 0, episode: 3 })); // 特典 E03
    println!("{}", describe(&EpisodeRef::Tvdb { season: 2, episode: 5 })); // 正篇 S2E05
    println!("{}", describe(&EpisodeRef::Tvdb { season: 9, episode: 1 })); // S9E01
    println!("{}", describe(&EpisodeRef::Absolute(366)));                  // 绝对 #366
    println!("{}", describe(&EpisodeRef::Absolute(999)));                  // 异常绝对集 #999
    println!("{}", describe(&EpisodeRef::Unknown));                        // 未知
}
```

⚠️ 三个坑：第一，把宽泛分支放前面（比如先写 `Absolute(n)` 再写 `Absolute(366)`），后面的分支报 **unreachable pattern** 警告，永远轮不到；第二，结构体模式里漏了字段又没写 `..`，报 **pattern does not mention field `season`**（E0027），加 `..` 或补上字段名即可；第三，用范围模式时写成 `1..=366` 就丢了绑定，后面再用 `n` 报 **cannot find value `n` in this scope**（E0425）——`@` 才是"范围 + 绑定"的那个符号。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 模式只能匹配常量 | 模式能绑定变量、解构结构体/元组、用 `\|` 合并、用范围匹配 |
| `@` 只是给模式改个名 | `@` 同时做"匹配"和"绑定"，`s @ (1 \| 2)` 里的 `s` 就是命中值本身 |
| 分支顺序无所谓 | 从上到下、第一个命中生效；宽泛分支放前面会让后面变 unreachable |
| 少写几个字段编译器会自动忽略 | 结构体模式必须写全字段或用 `..`，否则报 E0027 |

**在 tmdb-organizer 里**：第 13 周把 `EpisodeRef`（带季/集，或绝对集号）映射成新文件名时，就是这一天的模式大集合——特典（`season: 0`）、正篇（`season: s @ (1 | 2)`）、超范围绝对集各走一支，dry-run 打印时一眼能看出"这一集按哪种规则命名"。

### D33 · `if let` 与 `let else`

📖 [书 §6.3](https://kaisery.github.io/trpl-zh-cn/ch06-03-if-let.html)

**`if let` 是"只关心一种情况"的简写。** `if let Some(n) = ep.absolute { ... }` 等价于只写 `Some` 分支的 `match`，省掉了 `_ => {}` 那段样板。代价是**它放弃了穷尽性检查**：没写 `else` 时，`None` 会被静默忽略。所以 `if let` 适合"另一种情况不做事"的场合；需要明确处理两边时，还是 `match` 更清楚。

**变量绑定的作用域只在块内。** `if let Some(n)` 里的 `n` 出了 `{}` 就不存在了。想在外层继续用，就得在块内把要留下的值**赋给外层变量**，或者干脆用 `match` 把两个分支的值统一交出来。这个边界经常绊人，写之前先想清"我需要的值是块内还是块外"。

**`let else` 是"拿不到就提前退出"。** `let Some(x) = expr else { return; };` 让**主流程保持零缩进**：成功就继续往下，失败就在 `else` 里 `return` / `break` / `panic!`。它比 `if let` 好在"不缩进"，比 `match` 好在"不占半个屏幕"。前提是 `else` 块必须**发散**（走不回来），所以里面一般写 `return` 或 `continue`——这也是它和 `if let` 最本质的区别。

```rust
// if let 与 let else
#[derive(Debug)]
struct Episode {
    season: u32,
    episode: u32,
    title: String,
    absolute: Option<u32>,
}

fn main() {
    let ep = Episode {
        season: 1,
        episode: 1,
        title: String::from("死神来了"),
        absolute: Some(1),
    };

    // if let：只关心"有值"的那条路
    if let Some(n) = ep.absolute {
        println!("绝对集号：{n}"); // 绝对集号：1
    } else {
        println!("没有绝对集号");
    }

    // let else：拿不到就提前返回，主流程不缩进
    let Some(pos) = ep.title.find('死') else {
        println!("标题里没有关键字");
        return;
    };
    println!("关键字位置：{pos}"); // 关键字位置：0
}
```

⚠️ 三个坑：第一，`let else` 的 `else` 块没有发散（比如只 `println!` 而没有 `return`），报 **`else` clause of `let...else` does not diverge**（E0308）；第二，`if let` / `let else` 绑定的变量只在各自块内有效，块外再用报 **cannot find value `n` in this scope**（E0425）；第三，想不用 `else` 直接 `let Some(n) = opt;`，报 **refutable pattern in local binding**（E0005），编译器会提示 **you might want to use `let else` to handle the variant that isn't matched**——普通 `let` 只接受"必然成立"的模式。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `if let` 是 `match` 的完全等价简写 | 它放弃了穷尽性检查，另一种情况会被静默忽略 |
| `if let` 里绑定的变量块外还能用 | 绑定只在块内有效，块外就失效了 |
| `let else` 的 `else` 随便写什么都行 | 必须发散（`return` / `break` / `panic!`），否则报 E0308 |
| `let Some(x) = opt;` 不加 else 也行 | 不行，这是可反驳模式，报 E0005；要么加 `else`，要么改用 `match` / `if let` |

**在 tmdb-organizer 里**：第 8 周解析每个候选文件时会大量用 `if let Some((season, episode)) = parse_filename(name) { ... }`（解析成功才建 `AnimeFile`）；而第 14 周真正执行改名前的"安全检查"更适合 `let else`——`let Some(parent) = path.parent() else { continue; };`，拿不到父目录就直接跳过这个文件，主流程不缩进。

### D34 · 结构体 + 枚举组合建模

📖 [书 §6.1](https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html)

**枚举描述"是哪种情况"，结构体描述"一组一起出现的数据"。** `EpisodeOrder` 说清"按哪种顺序编号"，`RenamePlan` 把"旧名、新名、依据的顺序"捆在一起——单看 `String` 说不出哪个是旧名，而字段名本身就是文档。这个组合是第 13、14 周核心数据结构的雏形：**枚举当开关，结构体当载荷。**

**字段类型选择就是权限选择。** `from: String` / `to: String` 是**拥有**数据，计划一旦造好就与源文件名无关，生命周期清楚；若写成 `&str` 就得被外面的字符串"托着"，还会牵扯借用检查（第 6 周的生命周期）。这里的取舍很简单：**计划要活得比某个临时变量久，就让它拥有 `String`。**

**为什么不用元组 `(String, String, EpisodeOrder)`。** 元组能用，也不至于类型全错，但 `plan.0` 和 `plan.1` 谁是谁全靠记忆，两个 `String` 位置写反了编译器**根本不会报错**。字段名是零成本的自我说明——这也是 D5 那条"`file.episode` 比 `file.2` 强太多"在复合结构上的再次体现。

```rust
// 结构体 + 枚举组合建模：一个"重命名计划"由旧名、新名和依据的顺序组成
#[derive(Debug)]
enum EpisodeOrder {
    Tvdb,
    Absolute,
}

#[derive(Debug)]
struct RenamePlan {
    from: String,
    to: String,
    order: EpisodeOrder,
}

impl RenamePlan {
    fn new(from: &str, to: &str, order: EpisodeOrder) -> Self {
        Self { from: from.to_string(), to: to.to_string(), order }
    }
    fn describe(&self) -> String {
        format!("[{:?}] {} -> {}", self.order, self.from, self.to)
    }
}

fn main() {
    let plan = RenamePlan::new(
        "Bleach - 001.mkv",
        "Bleach.S01E001.mkv",
        EpisodeOrder::Absolute,
    );
    println!("{}", plan.describe());
    // [Absolute] Bleach - 001.mkv -> Bleach.S01E001.mkv
}
```

⚠️ 三个坑：第一，初始化漏字段，报 **missing field `order` in initializer of `RenamePlan`**（E0063）——所有字段都得给；第二，字段声明是 `String` 却直接塞 `&str` 字面量，报 **mismatched types: expected `String`, found `&str`**（E0308），要么 `.to_string()`，要么把字段类型改成 `&str`（但那样就要处理借用）；第三，`match self.order` 只写了部分变体，同样报 **non-exhaustive patterns**（E0004）。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 用元组更省事，字段名是多余 | 同类型字段的位置写反编译器不报错，字段名能防这类错 |
| 结构体字段顺序会影响正确性 | 不影响；初始化时字段顺序随意 |
| 结构体能随意嵌 `&str` 字段 | 能，但会引入借用/生命周期约束；本轮统一先用 `String` 拥有数据 |
| 枚举和结构体只能二选一 | 通常是组合：枚举当"开关"，结构体携带"这次操作的数据" |

**在 tmdb-organizer 里**：`RenamePlan { from, to, order }` 就是第 13 周 `plan.rs` 要落地的类型（真实版还会加 `path: PathBuf`，那是第 8 周的事）；`from`/`to` 用 `String`，正因为它要进入 `Vec<RenamePlan>` 被批量携带，不能借用临时变量。

### D35 · 复盘：让类型表达业务规则

📖 [书 §6.1](https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html)

**"让非法状态无法表示"是本周的总纲。** 把"新名字怎么拼"这条规则放进 `RenamePlan::new`，外部就**造不出**一个 `to` 字段和 `order` 对不上的计划——规则从"大家记得这么写"变成"类型只允许这么写"。这是 D29 到 D34 用到的一切工具（enum、struct、impl、match、`Option`）合起来的落点。

**回想 D14 的借用报错、D22 的 unwrap 风险、D31 的穷尽性，它们其实是同一件事。** 那就是：**把约束从运行期搬到编译期**。借用规则把"同时读写"变成编译错误，`Result` 把"忽略失败"变成编译错误，穷尽性把"忘了处理新情况"变成编译错误，受控构造器把"拼错名字"变成编译错误。第 4、5 周练的，就是识别这些"可以交给编译器的约束"。

**什么时候不这么做。** 受控构造器也有成本：字段私有化、多写一个 `new`、`main` 里不能再字面量构造。判据是**这条规则会不会被反复违反**——会，就值得收进类型；不会（比如一次性脚本里的临时结构），直接构造更省事。**前 21 天学的都是工具，从这里开始要学"什么时候不该滥用工具"。**

```rust
// 让类型表达业务规则：名字怎么拼由类型自己说了算，非法组合根本造不出来
#[derive(Debug)]
enum EpisodeOrder {
    Tvdb,
    Absolute,
}

#[derive(Debug)]
struct RenamePlan {
    from: String,
    to: String,
    order: EpisodeOrder,
}

impl RenamePlan {
    fn new(from: &str, series: &str, season: u32, episode: u32, order: EpisodeOrder) -> Self {
        let label = match order {
            EpisodeOrder::Tvdb => format!("S{season:02}E{episode:02}"),
            EpisodeOrder::Absolute => format!("E{episode:03}"),
        };
        Self { from: from.to_string(), to: format!("{series}.{label}.mkv"), order }
    }
}

fn main() {
    let a = RenamePlan::new("Bleach.S01E01.mkv", "Bleach", 1, 1, EpisodeOrder::Tvdb);
    let b = RenamePlan::new("Bleach - 366.mkv", "Bleach", 1, 366, EpisodeOrder::Absolute);
    println!("{} -> {}（{:?}）", a.from, a.to, a.order);
    println!("{} -> {}（{:?}）", b.from, b.to, b.order);
    // Bleach.S01E01.mkv -> Bleach.S01E01.mkv（Tvdb）
    // Bleach - 366.mkv -> Bleach.E366.mkv（Absolute）
}
```

⚠️ 三个坑：第一，`new` 里构造 `Self` 漏了字段，报 **missing field ... in initializer**（E0063）；第二，给 `RenamePlan` 加 `#[derive(PartialEq)]` 但 `EpisodeOrder` 没派生，比较时报 **`EpisodeOrder` doesn't implement `PartialEq`**（E0277）——派生要顺着字段类型一路补齐；第三，以为 `RenamePlan::new` 一定比直接构造"更安全"——如果 `new` 里也塞了 `unwrap`，那只是把风险换了个地方，**规则要真的收进类型，而不是收进一个会崩的函数。**

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 业务规则靠注释和 code review 保证 | 能搬进类型的就搬进去，让编译器来守 |
| 受控构造器越多越好 | 有成本（字段私有、多写代码）；只对"会被反复违反"的规则值得 |
| `new` 里 `unwrap` 和外面 `unwrap` 差不多 | 差不多，都没解决问题；受控构造器要配合 `Result` / `Option` 才算收口 |
| 元组和结构体表达力一样 | 结构体字段名能防位置写错；业务模型优先用结构体 |

**在 tmdb-organizer 里**：第 13 周 `plan.rs` 会提供 `RenamePlan::new`（或 `build_plan`），把"按 `order` 拼新名"固定在一处；第 14 周 dry-run 只是把 `Vec<RenamePlan>` 打出来给用户确认；第 15 周再接 `anyhow` 把错误收口。**这一周结束时，你已经能把"文件整理"的业务规则用类型和枚举描述清楚，剩下的只是接数据源和落盘。**

---

<a id="w6"></a>

## 第 6 周：泛型、Trait、生命周期基础

| 天   | 学 6 min                                  | 写 10 min                                                    | 验收 / commit          |
| ---- | ----------------------------------------- | ------------------------------------------------------------ | ---------------------- |
| D36  | 泛型函数与泛型参数（[书 §10.1](https://kaisery.github.io/trpl-zh-cn/ch10-01-syntax.html)） | `fn largest<T: PartialOrd>(list: &[T]) -> &T`                | 用 u32 测试，`day36`   |
| D37  | `trait` 定义与默认实现（[书 §10.2](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html)） | `trait EpisodeFormatter { fn format(&self) -> String; }`     | 定义 trait，`day37`    |
| D38  | 为具体类型实现 `trait`（[书 §10.2](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html)） | 为 `AnimeFile` 实现 `EpisodeFormatter`                       | 调用 `format`，`day38` |
| D39  | trait bound 与 `impl Trait`（[书 §10.2](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html)） | `fn print_formatted<T: EpisodeFormatter>(item: &T)`          | 打印，`day39`          |
| D40  | 生命周期标注 `'a`（[书 §10.3](https://kaisery.github.io/trpl-zh-cn/ch10-03-lifetime-syntax.html)） | `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str`          | 测试通过，`day40`      |
| D41  | `dyn Trait` trait 对象（[书 §18.2](https://kaisery.github.io/trpl-zh-cn/ch18-02-trait-objects.html)） | `trait EpisodeSource { fn episodes(&self, order: &EpisodeOrder) -> Vec<Episode>; }` | fake 实现，`day41`     |
| D42  | 复盘：什么时候该用泛型、什么时候用 trait 对象 | 用 trait 抽象输出，替换硬编码打印                            | `cargo run`，`day42`   |

### D36 · 泛型函数与泛型参数

📖 [书 §10.1](https://kaisery.github.io/trpl-zh-cn/ch10-01-syntax.html)

**泛型的本质是"编译期填空"。** `fn largest<T>(list: &[T])` 里的 `T` 不是运行期存在的类型，而是给编译器留的**占位符**：你调用 `largest(&episodes)` 时它被填成 `u32`，编译器为这个组合生成一份专门的机器码。这个过程叫**单态化**（monomorphization），代价是"用几次就生成几份代码"（编译慢一点、二进制大一点），收益是运行时**零额外开销**——泛型版和你手写 `largest_u32` 生成的代码完全一样。

**为什么必须写 `T: PartialOrd`。** 函数体对 `T` 一无所知，编译器只允许你用"契约里写明了的"能力。`item > max` 里的 `>` 来自 `PartialOrd`，不写约束就报 **binary operation `>` cannot be applied to type `&T`**（E0369），并提示 **consider restricting type parameter `T` with trait `PartialOrd`**。这条约束是函数对调用方的承诺：只接受能比大小的类型。D3 说的"签名是边界"，在泛型上就体现为 bound——**bound 写得越准，报错来得越早**。

| 写法 | 决定时刻 | 运行期代价 | 什么时候用 |
| ---- | ---- | ---- | ---- |
| `<T: PartialOrd>` 单态化 | 编译期 | 零，可内联 | 类型集合已知、性能敏感 |
| `&dyn PartialOrd` 动态分发 | 运行期 | 每次调用查一次 vtable | 需要混装多种实现（见 D41） |

```rust
// 泛型函数：一份代码适配多种类型，编译器为每个具体类型各生成一份
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in list {
        if item > max {
            max = item;
        }
    }
    max
}

fn main() {
    let episodes: Vec<u32> = vec![1, 366, 12, 57];
    println!("最大集号：{}", largest(&episodes)); // 最大集号：366

    let sizes: Vec<f64> = vec![0.9, 1.4, 1.2];
    println!("最大体积：{}", largest(&sizes)); // 最大体积：1.4

    let names = vec!["Bleach", "Naruto"];
    println!("字典序最大：{}", largest(&names)); // 字典序最大：Naruto
}
```

⚠️ 三个坑：第一，漏写 `PartialOrd` 约束，报 **binary operation `>` cannot be applied to type `&T`**（E0369）；第二，`&list[0]` 遇到**空切片**会 panic 报 `index out of bounds`，泛型不负责边界，得先判 `is_empty()`；第三，返回的是 `&T` 而不是 `T`，说明结果**借用了 `list`**——泛型并没有绕过 D8 的所有权与借用规则。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 泛型会像 Java 那样运行期擦除类型 | 单态化：每个具体类型生成独立代码，运行时没有泛型信息残留 |
| 泛型函数比具体类型版本慢 | 单态化后与手写 `largest_u32` 等价，零开销 |
| 泛型参数能随便做任何操作 | 只能用 trait bound 声明过的能力，否则编译报错 |
| 泛型参数名必须叫 `T` | 名字随意（`T` / `Item` / `N`），单大写字母只是社区习惯 |

**在 tmdb-organizer 里**：第 13 周 `plan.rs` 的 `fn build_plan<T: EpisodeSource>(source: &T, ...)` 会先用泛型表达"任何数据源都能生成计划"；等 D41 需要把多个数据源混装进一个 `Vec`，才换成 `dyn`。**今天先用泛型把规则写清，别急着上 trait 对象。**

### D37 · `trait` 定义与默认实现

📖 [书 §10.2](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html)

**trait 是"行为契约"，不是"数据布局"。** `struct` 说"我有哪些字段"，`trait` 说"我能做哪些事"。`trait EpisodeFormatter` 只声明 `fn label(&self) -> String`，完全不关心实现者长什么样；任何类型——`AnimeFile`、`Episode`、甚至 `String`——只要补上这个方法，就"是"一个 `EpisodeFormatter`。这让**调用方可以只依赖行为、不依赖具体类型**，第 6 周所有抽象都从这里长出来。

**默认实现是为了"共同逻辑只写一遍"。** 带方法体的 trait 方法就是默认实现：实现者不写就白拿，写了就**覆写**。`format()` 用 `[{label}]` 拼好，各类型只需提供 `label()`。但要注意和继承的差别：Rust 没有继承，默认实现**看不见实现者的字段**，它只能调用 trait 上声明的其他方法——这反过来逼你把"最小必需方法"设计清楚。**默认实现能省代码，但它不该依赖任何实现细节。**

**为什么 trait 方法几乎都带 `&self`。** `fn label(&self) -> String` 里的 `&self` 就是 D6 的借用规则：trait 不认识实现者，只能通过 `self` 访问它；写 `&self` 表示"只读看一眼"，这也是绝大多数 trait 方法的形态。要改数据用 `&mut self`，要消费就写 `self`——选择标准和 D6 一个字都没变。

```rust
// trait 是一份行为契约；带默认实现的方法让实现者少写代码
struct AnimeFile { name: String, season: u32, episode: u32 }
struct Episode { season: u32, episode: u32 }

trait EpisodeFormatter {
    fn label(&self) -> String;                  // 必需：各类型自己实现
    fn format(&self) -> String {                // 默认实现：可覆写，也可不写
        format!("[{}]", self.label())
    }
}

impl EpisodeFormatter for AnimeFile {
    fn label(&self) -> String {
        format!("{} S{:02}E{:02}", self.name, self.season, self.episode)
    }
}

impl EpisodeFormatter for Episode {
    fn label(&self) -> String { format!("S{:02}E{:02}", self.season, self.episode) }
    fn format(&self) -> String { format!("《死神》{}", self.label()) } // 覆写默认实现
}

fn main() {
    let f = AnimeFile { name: String::from("Bleach"), season: 1, episode: 1 };
    let e = Episode { season: 1, episode: 366 };
    println!("{}", f.format()); // 没写 format：走默认实现 [Bleach S01E01]
    println!("{}", e.format()); // 写了 format：走覆写 《死神》S01E366
}
```

⚠️ 三个坑：第一，`impl EpisodeFormatter for AnimeFile {}` 不提供 `label`，报 **not all trait items implemented, missing: label**（E0046）；第二，给 trait 加了必需方法后，所有实现者都要补上，否则每个实现处都会报同一条 E0046；第三，默认实现里直接访问 `self.name` 会报 **no field `name` on type `Self`**——trait 不知道实现者有什么字段，只能调已声明的方法。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| trait 像抽象基类，能有字段和构造函数 | trait 只声明行为，不能有字段，也没有构造函数的概念 |
| 默认实现的方法不能在实现块里改 | 可以覆写，实现块里的版本优先 |
| trait 方法必须都写 `&self` | 也可以是 `&mut self` / `self`，或干脆没有 self 的关联函数 |
| 一个类型只能实现一个 trait | 可以实现任意多个，各 trait 的名字空间互相独立 |

**在 tmdb-organizer 里**：第 12 周会给 `Episode`（打印 TMDB 返回的标题）和 `RenamePlan`（打印 `旧名 -> 新名`）都实现 `EpisodeFormatter`；第 14 周 dry-run 的输出就靠它，因为 dry-run 需要的是"能打印"，而不是某个具体类型。

### D38 · 为具体类型实现 `trait`

📖 [书 §10.2](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html)

**`impl Trait for Type` 是"补契约"，不改变类型本身。** 实现 trait 不会给类型加字段、加构造函数，只是让它在**类型系统**里多一层身份。所以同一份数据可以同时实现多个 trait（`EpisodeFormatter` + `Debug` + `Display`），互不干扰；也可以给**别人定义的类型**实现 trait，只要其中一方是本 crate 的。

**trait 的价值不在实现侧，而在调用侧。** 三个类型各自实现 `EpisodeFormatter` 之后，D39 的泛型函数就能对它们一视同仁。反过来，如果每个类型各写一个自由函数（`format_file` / `format_episode`），调用方必须记住每个函数名，也没法用一个签名统一处理——**接口一致，才换来"闭眼看类型"的能力。**

**孤儿规则是在保护"一致性"。** 给 `Vec<u8>` 实现 `Display` 会报 **only traits defined in the current crate can be implemented for types defined outside of the crate**（E0117）。原因很实在：如果两个库都能给 `Vec<u8>` 加 `Display`，第三个库调用时编译器就不知道该用哪份，升级依赖时会突然编译失败。规则一句话——**trait 和类型至少有一个必须是本 crate 定义的**。

```rust
// 为具体类型实现 trait：接口一致、实现各异
struct AnimeFile { name: String, season: u32, episode: u32 }

trait EpisodeFormatter {
    fn label(&self) -> String;
    fn format(&self) -> String { self.label() } // 默认实现
}

impl EpisodeFormatter for AnimeFile {
    fn label(&self) -> String {
        format!("{} S{:02}E{:02}", self.name, self.season, self.episode)
    }
}

impl EpisodeFormatter for (u32, u32) { // (季, 集) 元组也能成为格式化器
    fn label(&self) -> String { format!("S{:02}E{:02}", self.0, self.1) }
}

fn main() {
    let f = AnimeFile { name: String::from("Bleach"), season: 1, episode: 1 };
    println!("{}", f.format());       // Bleach S01E01
    println!("{}", (1, 366).label()); // S01E366
}
```

⚠️ 三个坑：第一，实现块写了方法名但**忘了写 `impl Trait for Type` 那行头**（或拼错 trait 名），方法会变成类型自己的普通方法，调用处报 **no method named `label` found for struct `AnimeFile`**（E0599）；第二，trait 没 `use` 进来时报同一条 E0599，并提示 **items from traits can only be used if the trait is in scope**（D45 细说）；第三，给标准库类型实现标准库 trait 报 **only traits defined in the current crate can be implemented for types defined outside of the crate**（E0117）。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 实现 trait 会给类型加功能代码 | 只是让类型在编译期多一层身份，数据布局不变 |
| 给标准库类型实现标准库 trait 是常规操作 | 被孤儿规则挡住（E0117），必须有一方属于本 crate |
| `impl` 块必须和类型写在同一个文件 | 不必，另一个文件里写 `impl EpisodeFormatter for AnimeFile` 完全合法 |
| 元组这类"匿名"类型不能实现 trait | 可以，(u32, u32) 也能实现，只要不合孤儿规则的忌讳 |

**在 tmdb-organizer 里**：第 13 周 `plan.rs` 里 `impl EpisodeFormatter for RenamePlan` 会把 `旧名 -> 新名` 的拼法固定在一个地方；第 15 周 `rename.rs` 的 dry-run 只依赖这个 trait，**换输出方式时不用碰数据类型本身**。

### D39 · trait bound 与 `impl Trait`

📖 [书 §10.2](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html)

**trait bound 是"给泛型参数划能力范围"。** `fn print_formatted<T: EpisodeFormatter>(item: &T)` 里的 `T: EpisodeFormatter` 就是 bound：`T` 可以是任何类型，但必须实现这个 trait。它和 D36 的 `<T: PartialOrd>` 是同一机制，只是约束的 trait 不同；多个约束用 `+` 连接（`T: EpisodeFormatter + Clone`），签名长时还可以用 `where` 子句挪到后面，读起来更清楚。

**`impl Trait` 是"匿名泛型"，只换了写法。** 参数位的 `fn print_impl(item: &impl EpisodeFormatter)` 与 `fn print_formatted<T: EpisodeFormatter>(item: &T)` 编译后**完全等价**。区别只有一个：`impl Trait` 不给参数起名，所以**同一个类型出现两次就写不出来**——`fn f(x: impl Trait, y: impl Trait)` 是两个不同类型，而 `<T: Trait>(x: T, y: T)` 要求同类型。所以规则是：只用一个参数就 `impl Trait`，要表达"两个参数同类型"就用显式泛型参数。

**返回位的 `impl Trait` 是另一回事：它在承诺"我返回了某个实现了该 trait 的类型"。** `fn make_file() -> impl EpisodeFormatter` 让调用方只能把它当 `EpisodeFormatter` 用，看不到具体类型——这正是隐藏实现细节的手段。代价有两个：调用方无法再用返回值的具体类型；而且它不能用在变量标注上，`let x: impl EpisodeFormatter = ...` 报 **`impl Trait` is not allowed in the type of variable bindings**（E0562）。

```rust
// trait bound 约束泛型参数能做哪些事；impl Trait 只是同一件事的简写
struct AnimeFile { name: String, season: u32, episode: u32 }

trait EpisodeFormatter {
    fn label(&self) -> String;
}

impl EpisodeFormatter for AnimeFile {
    fn label(&self) -> String { format!("{} S{:02}E{:02}", self.name, self.season, self.episode) }
}

fn print_formatted<T: EpisodeFormatter>(item: &T) { // 显式泛型参数 + bound
    println!("{}", item.label());
}

fn print_impl(item: &impl EpisodeFormatter) { // 等价写法，省掉给参数起名
    println!("{}", item.label());
}

fn make_file() -> impl EpisodeFormatter { // 返回位：隐藏具体类型
    AnimeFile { name: String::from("Bleach"), season: 1, episode: 1 }
}

fn main() {
    let f = AnimeFile { name: String::from("Bleach"), season: 1, episode: 366 };
    print_formatted(&f);                 // Bleach S01E366
    print_impl(&f);                      // Bleach S01E366
    println!("{}", make_file().label()); // Bleach S01E01
}
```

⚠️ 三个坑：第一，对没实现该 trait 的类型调用，报 **the trait bound `Episode: EpisodeFormatter` is not satisfied**（E0277），并提示要加哪个 bound；第二，`let x: impl EpisodeFormatter = ...` 报 **`impl Trait` is not allowed in the type of variable bindings**（E0562）——`impl Trait` 只能出现在参数位和返回位；第三，把 `impl Trait` 写进结构体字段（`struct Holder { item: impl EpisodeFormatter }`）同样不合法，要么用泛型参数 `Holder<T>`，要么用 `Box<dyn EpisodeFormatter>`（D41）。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `impl Trait` 是动态分发、有运行期开销 | 参数位/返回位的 `impl Trait` 仍是单态化，编译期定死具体类型 |
| `impl Trait` 和泛型参数是两种东西 | 参数位两者完全等价，只是一个匿名、一个能起名并复用 |
| `impl Trait` 可以到处用（字段、局部变量） | 只允许参数位和返回位，其余地方报 E0562 |
| 返回 `impl Trait` 时调用方能拿到具体类型 | 拿不到，只能按 trait 用；这是"隐藏实现"的代价与目的 |

**在 tmdb-organizer 里**：第 13 周 `build_plan` 的参数用 `&impl EpisodeSource` 就够（只看一个数据源）；第 15 周想让 `main` 完全不依赖具体实现时，才把 `make_source()` 写成 `-> impl EpisodeSource`，**让"用哪个数据源"成为函数内部的私事**。

### D40 · 生命周期标注 `'a`

📖 [书 §10.3](https://kaisery.github.io/trpl-zh-cn/ch10-03-lifetime-syntax.html)

**生命周期标注不是在"延长"任何东西，它只是在描述"引用之间的关联"。** `fn longest<'a>(x: &'a str, y: &'a str) -> &'a str` 的意思是"返回的引用活得不超过 `x` 和 `y` 中较短的那个"。引用本身真正活多久，仍由**它指向的数据**决定——`'a` 一个字节都不改变。这是最容易误解的一点：**标注是描述，不是指令。**

**为什么编译器需要这个描述。** 借用检查器要保证"返回的引用在被使用期间，它指向的数据还活着"。对 `fn longest(x: &str, y: &str) -> &str`，编译器看不到输入输出的关系，不知道该拿谁约束返回值，于是报 **missing lifetime specifier**（E0106）并提示 **expected named lifetime parameter**。写 `'a` 就是把答案摆出来。另一条省事的路是**省略规则**：只有一个输入引用时，返回值自动取它的生命周期；有 `&self` 时自动取 `self` 的——所以 D6 的 `fn display_label(&self) -> &str` 根本不用写标注。

**结构体持有引用也必须标注。** `struct SeriesView<'a> { name: &'a str }` 的含义是"这个结构体不能比它借的字符串活得久"；实现块上还要再写一遍（`impl<'a> SeriesView<'a>`）。一个直接推论是：**返回 `&str` 的函数不能返回函数内部新建的 `String`**——`&s` 会报 **cannot return reference to local variable**（E0515），因为函数结束时 `s` 已被释放。这时只能返回拥有所有权的 `String`，正是 D6 那条判定的再次出现。

```rust
// 生命周期标注：描述"引用之间的关联"，不改变引用实际活多久
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() >= y.len() { x } else { y }
}

struct SeriesView<'a> { // 结构体持有借用，字段上必须写出生命周期
    name: &'a str,
    first_file: &'a str,
}

fn main() {
    let name = String::from("Bleach");
    let file = String::from("Bleach.S01E01.mkv");
    let view = SeriesView { name: &name, first_file: &file };
    println!("{} 首集文件：{}", view.name, view.first_file);
    // Bleach 首集文件：Bleach.S01E01.mkv

    let a = String::from("Bleach.S01E01.mkv");
    let b = String::from("Bleach.S01E366.mkv");
    println!("{}", longest(&a, &b)); // Bleach.S01E366.mkv
}
```

⚠️ 三个坑：第一，`fn longest(x: &str, y: &str) -> &str` 报 **missing lifetime specifier**（E0106），并提示 **this function's return type contains a borrowed value, but the signature does not say whether it is borrowed from x or y**；第二，`struct SeriesView { name: &str }` 漏标注同样报 E0106，提示 **expected named lifetime parameter**；第三，返回内部新建字符串的引用报 **cannot return reference to local variable**（E0515），附注 **returns a reference to data owned by the current function**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `'a` 能延长引用的存活时间 | 它只是关系声明，不改任何值的实际生命周期 |
| 所有函数都得写生命周期标注 | 绝大多数不用写，靠省略规则；只在"输入多个引用且要返回引用"时才必须显式 |
| 两个输入的 `'a` 必须是同一个具体生命周期 | 实际取的是调用时两者的**交集**（较短的那个） |
| `'static` 是"永不释放"的全局变量 | 它表示"活得和整个程序一样久"，字符串字面量天然满足 |

**在 tmdb-organizer 里**：第 8 周 `scan_dir` 返回的是 `Vec<PathBuf>`（拥有数据），所以基本不碰生命周期；真正会用到的是第 13 周想从 `AnimeFile` 里"借出"一段名字切片时——**先按"返回 `String` 最省事"来写，只有确认是性能热点，才改用带 `'a` 的借用版本。**

### D41 · `dyn Trait` trait 对象

📖 [书 §18.2](https://kaisery.github.io/trpl-zh-cn/ch18-02-trait-objects.html)

**trait 对象解决的是"运行期才知道具体类型"。** `Box<dyn EpisodeSource>` 里装的是一个**胖指针**：一个指针指向数据，另一个指向**虚表**（vtable，记录该类型每个方法的地址）。调用 `s.episodes(...)` 时程序先去 vtable 查到函数地址再跳过去——这叫**动态分发**。它能做到泛型做不到的事：把 `FakeSource` 和 `TmdSource` 装进**同一个 `Vec`**，运行期决定调哪个实现。

| 维度 | `<T: Trait>` 单态化 | `dyn Trait` 动态分发 |
| ---- | ---- | ---- |
| 决定时刻 | 编译期 | 运行期 |
| 代码体积 | 每个具体类型一份 | 只有一份 |
| 调用开销 | 零，可内联 | 一次 vtable 查表，通常无法内联 |
| 能否混装不同类型 | 不能，`Vec<T>` 只能一种 | 能，`Vec<Box<dyn Trait>>` |
| 对大小的要求 | 需要 `Sized` | 不需要，所以必须套 `Box` 或 `&` |

**为什么必须套一层 `Box` 或 `&`。** `dyn EpisodeSource` 是**不定长类型**（DST）：编译器不知道具体实现占多少字节，所以不能直接当变量类型——`let s: dyn EpisodeSource = Fake;` 报 **the size for values of type `dyn EpisodeSource` cannot be known at compilation time**（E0277）。`Box<dyn Trait>`（拥有）和 `&dyn Trait`（借用）本身是**固定大小的指针**，把"大小不定"隔离在指针后面，问题就解决了。同一原因，集合里要装 trait 对象就只能装 `Box<dyn Trait>`。

**trait 对象有"dyn 兼容"的门槛。** 不是所有 trait 都能当对象用：带泛型方法的 trait（`fn pick<T>(&self, ...)`）进不了 vtable，因为每个 `T` 都是不同的函数。此时报 **the trait `EpisodeSource` is not dyn compatible**（E0038）。记住这条取舍——**trait 一旦要当对象用，方法签名里就不能有泛型参数。**

```rust
// trait 对象：不同实现装进同一个集合，运行期经 vtable 分发
enum EpisodeOrder { Tvdb, Absolute }
struct Episode { season: u32, episode: u32 }

trait EpisodeSource {
    fn episodes(&self, order: &EpisodeOrder) -> Vec<Episode>;
}

struct FakeSource; // 离线假数据源：本地开发和测试都用它
impl EpisodeSource for FakeSource {
    fn episodes(&self, order: &EpisodeOrder) -> Vec<Episode> {
        let n = match order { EpisodeOrder::Tvdb => 1, EpisodeOrder::Absolute => 366 };
        vec![Episode { season: 1, episode: n }]
    }
}

struct TmdSource(String); // 第 12 周真正接 TMDB 的位置
impl EpisodeSource for TmdSource {
    fn episodes(&self, _order: &EpisodeOrder) -> Vec<Episode> {
        println!("向 TMDB 查询 {} 的剧集……", self.0);
        vec![Episode { season: 1, episode: 2 }] // 占位：真实实现会发 HTTP 请求
    }
}

fn main() {
    // 两种不同实现装进同一个 Vec——这是泛型做不到的事
    let sources: Vec<Box<dyn EpisodeSource>> = vec![
        Box::new(FakeSource),
        Box::new(TmdSource(String::from("Bleach"))),
    ];
    for s in &sources {
        let eps = s.episodes(&EpisodeOrder::Tvdb);
        println!("S{:02}E{:02}", eps[0].season, eps[0].episode);
    }
    let abs = FakeSource.episodes(&EpisodeOrder::Absolute);
    println!("绝对集号：{}", abs[0].episode);
    // S01E01
    // 向 TMDB 查询 Bleach 的剧集……
    // S01E02
    // 绝对集号：366
}
```

⚠️ 三个坑：第一，直接写 `let s: dyn EpisodeSource = Fake;` 报 **the size for values of type `dyn EpisodeSource` cannot be known at compilation time**（E0277），并提示可以装箱（**you could box the found value and coerce it to the trait object**）；第二，trait 里带了泛型方法后当对象用，报 **the trait `EpisodeSource` is not dyn compatible**（E0038）；第三，`Box<dyn Trait>` 调用方法时是"经过指针的间接调用"，`&dyn Trait` 需要生命周期不短于借用——**能返回 `&dyn Trait` 的情况很少，多数时候直接返回 `Box<dyn Trait>`。**

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `dyn Trait` 是"无类型的动态对象" | 它是胖指针（数据指针 + vtable 指针），类型检查在编译期已完成 |
| 用 `dyn` 就不用写 trait 了 | 必须有 trait，`dyn` 只是"以 trait 为类型"的写法 |
| trait 对象能替代所有泛型 | 它有 vtable 查表开销、不能内联、还会挡住部分优化 |
| 任意 trait 都能写成 `dyn Trait` | 只有 dyn 兼容的 trait 才行，带泛型方法或返回 `Self` 的都不行 |

**在 tmdb-organizer 里**：第 12 周 `FakeSource`（读本地 JSON 假数据）和 `TmdSource`（发 HTTP）会各实现一次 `EpisodeSource`，第 13 周的 `build_plan` 只依赖这个 trait；第 15 周 `main` 里按 `--offline` 开关决定塞哪个进 `Vec<Box<dyn EpisodeSource>>`——**这正是 D41 这张表的用处。**

### D42 · 复盘：什么时候该用泛型、什么时候用 trait 对象

📖 [书 §10.2](https://kaisery.github.io/trpl-zh-cn/ch10-02-traits.html)、[书 §18.2](https://kaisery.github.io/trpl-zh-cn/ch18-02-trait-objects.html)

**判断标准只有一条：类型集合是"编译期已知"还是"运行期才知道"。** 只看一个类型参数、性能敏感、希望被内联的路径用泛型（单态化）；需要**在一个集合里混装多种实现**，或者运行期根据开关决定用哪个实现，用 `dyn Trait`。D41 的 `Vec<Box<dyn EpisodeSource>>` 属于后者，`fn build_plan<T: EpisodeSource>` 属于前者——**同一个工具两种形态，选错只是多付一点性能或一点灵活度，不会出错，但代码会暴露你的判断。**

**trait 真正的收益是"换实现不用改调用方"。** 把硬编码的 `println!` 换成 `Reporter` trait 之后，`main` 与"怎么输出"解耦：dry-run 用 `ConsoleReporter`，加 `--quiet` 时换 `SummaryReporter`，`main` 一行都不用动。这就是依赖倒置的朴素形态——**调用方依赖抽象的 trait，而不是依赖某个具体的打印方式。**

**但别为了"未来可能替换"提前抽象。** 抽象有成本：多一个 trait、多一个泛型参数、错误信息变长、阅读时要多点一层跳转。判据是**现在就有两个实现，或已经确定要接第二个**（比如 D41 的假数据源 + 真实数据源）。只有一个实现时，直接写具体类型更清楚。**这是第 5 周"什么时候不该滥用类型"的续集：工具都会了，接下来学的是"什么时候放着不动"。**

```rust
// 复盘：同一件事，泛型走编译期单态化，dyn 走运行期 vtable 查表
struct RenamePlan { from: String, to: String }

trait Reporter {
    fn report(&self, plans: &[RenamePlan]);
}

struct ConsoleReporter; // 逐个打印：dry-run 默认用它
impl Reporter for ConsoleReporter {
    fn report(&self, plans: &[RenamePlan]) {
        for p in plans { println!("{} -> {}", p.from, p.to); }
    }
}

struct SummaryReporter; // 只报数量：--quiet 时用它
impl Reporter for SummaryReporter {
    fn report(&self, plans: &[RenamePlan]) {
        println!("共 {} 个文件待整理", plans.len());
    }
}

fn run<R: Reporter>(reporter: &R, plans: &[RenamePlan]) { // 泛型：编译期定死实现
    reporter.report(plans);
}

fn main() {
    let plans = vec![
        RenamePlan { from: String::from("Bleach - 001.mkv"), to: String::from("Bleach.E001.mkv") },
    ];
    run(&ConsoleReporter, &plans);
    // Bleach - 001.mkv -> Bleach.E001.mkv

    let reporters: Vec<Box<dyn Reporter>> = // 两种实现装进同一个 Vec
        vec![Box::new(ConsoleReporter), Box::new(SummaryReporter)];
    for r in &reporters {
        r.report(&plans); // 运行期才知道调谁
    }
    // Bleach - 001.mkv -> Bleach.E001.mkv
    // 共 1 个文件待整理
}
```

⚠️ 三个坑：第一，把 `fn run<R: Reporter>` 写成 `fn run(r: &dyn Reporter)` 不会报错，但会丢掉单态化——这是**风格与性能的取舍**，不是对错问题；第二，`Vec<Box<dyn Reporter>>` 里想塞 `&ConsoleReporter` 会因生命周期不够长而报 **borrowed value does not live long enough**，要么改成 `Box::new`，要么统一用 `Vec<&dyn Reporter>` 并保证被借的值活得够久；第三，trait 方法返回 `Self` 或带泛型参数会让 trait 不再 dyn 兼容，此时只能退回泛型。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 泛型比 trait 对象"更高级" | 两者解决不同问题：泛型求性能，trait 对象求灵活 |
| 用了 trait 就必须写 `dyn` | 不必，`T: Trait` / `impl Trait` 都是泛型路线，没有运行期开销 |
| 抽象越多越好，方便以后扩展 | 抽象有阅读和维护成本，只有一个实现时直接写具体类型 |
| `dyn` 只是语法糖，没代价 | 有 vtable 查表开销、不可内联、还会挡住部分跨函数优化 |

**在 tmdb-organizer 里**：第 14 周的 dry-run 输出、第 15 周的 `--quiet` 都走这条 `Reporter` 思路；而 `build_plan` 内部仍用泛型，因为它只服务于一个数据源、且是热路径。**下一周开始拆文件，`models.rs` / `parser.rs` 会把本周的 trait 定义各归其位。**

---

<a id="w7"></a>

## 第 7 周：模块、测试、文档，开始拆文件

| 天   | 学 6 min                                | 写 10 min                                       | 验收 / commit         |
| ---- | --------------------------------------- | ----------------------------------------------- | --------------------- |
| D43  | 模块拆分到文件（[书 §7.5](https://kaisery.github.io/trpl-zh-cn/ch07-05-separating-modules-into-different-files.html)） | 新建 `src/models.rs`，放 `AnimeFile`、`Episode` | 编译通过，`day43`     |
| D44  | 模块定义与 `pub` 可见性（[书 §7.2](https://kaisery.github.io/trpl-zh-cn/ch07-02-defining-modules-to-control-scope-and-privacy.html)） | 新建 `src/parser.rs`，放 `parse_filename`       | 编译通过，`day44`     |
| D45  | 路径与 `use` 引入（[书 §7.3](https://kaisery.github.io/trpl-zh-cn/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)、[§7.4](https://kaisery.github.io/trpl-zh-cn/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)） | `main.rs` 写 `mod models; mod parser; use ...`  | 调用成功，`day45`     |
| D46  | 单元测试与 `#[cfg(test)]`（[书 §11.1](https://kaisery.github.io/trpl-zh-cn/ch11-01-writing-tests.html)） | 在 `parser.rs` 写 `#[cfg(test)] mod tests`      | `cargo test`，`day46` |
| D47  | 集成测试的组织结构（[书 §11.3](https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html)） | 新建 `tests/parser_test.rs`                     | 测试公共 API，`day47` |
| D48  | `should_panic` 与测试筛选（[书 §11.2](https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html)） | 补 3 个边界测试：缺季、缺集、乱码               | 全绿，`day48`         |
| D49  | 文档注释与 `cargo fmt`（[书 §14.2](https://kaisery.github.io/trpl-zh-cn/ch14-02-publishing-to-crates-io.html)、[附录 D](https://kaisery.github.io/trpl-zh-cn/appendix-04-useful-development-tools.html)） | 给公共函数写 `///`，`cargo fmt`                 | `cargo test`，`day49` |

### D43 · 模块拆分到文件

📖 [书 §7.5](https://kaisery.github.io/trpl-zh-cn/ch07-05-separating-modules-into-different-files.html)

**模块是"命名空间 + 可见性边界"，文件只是它的载体。** `mod models;` 这句声明告诉编译器"去找 `src/models.rs`，把里面的东西装进 `models` 模块"。关键认识是：**模块树由 `mod` 声明决定，不由文件位置决定**——文件放哪、叫什么，只决定 `mod` 去哪里找内容；`models.rs` 里的 `AnimeFile` 全名是 `crate::models::AnimeFile`，和文件系统路径没有必然对应。所以"拆文件"只需动两处：新文件放代码，`main.rs` 加一行 `mod`。

**为什么值得拆。** D1-D42 的代码全堆在 `main.rs` 里，第 8 周要接文件系统、第 10 周要发 HTTP、第 13 周要算重命名计划，它会很快膨胀到几百行，找 `parse_filename` 只能靠搜索。拆成 `models.rs`（数据结构）和 `parser.rs`（解析逻辑）后，**每个文件只回答一个问题**；依赖方向也变清楚了：`parser.rs` 不认识 `AnimeFile`，`models.rs` 不认识解析逻辑，谁也不该反过来依赖 `main.rs`。

**`src/models.rs` 和 `src/models/mod.rs` 是同一个模块的两种写法。** 单文件用前者；内容多了要继续拆时，改成 `src/models/mod.rs` + `src/models/*.rs`，而**外层的 `mod models;` 一个字都不用改**。"模块路径稳定、文件布局可变"这条性质，是 Rust 敢让你放心重构的原因之一。

src/models.rs
```rust
// 模型层：只放数据结构与它们自己的方法
#[derive(Debug, Clone)]
pub struct AnimeFile {
    pub name: String,
    pub season: u32,
    pub episode: u32,
}

#[derive(Debug, Clone)]
pub struct Episode {
    pub season: u32,
    pub episode: u32,
    pub title: String,
}

impl AnimeFile {
    pub fn label(&self) -> String {
        format!("{} S{:02}E{:02}", self.name, self.season, self.episode)
    }
}

impl Episode {
    pub fn tag(&self) -> String {
        format!("S{:02}E{:02} {}", self.season, self.episode, self.title)
    }
}
```

src/main.rs
```rust
mod models; // 声明子模块：编译器会去找 src/models.rs

fn main() {
    let f = models::AnimeFile { name: String::from("Bleach"), season: 1, episode: 1 };
    let ep = models::Episode { season: 1, episode: 366, title: String::from("死神来了") };
    println!("{}", f.label()); // Bleach S01E01
    println!("{}", ep.tag()); // S01E366 死神来了
}
```

⚠️ 三个坑：第一，写了 `mod models;` 却没有对应文件，报 **file not found for module `models`**（E0583），提示去找 `src/models.rs` 或 `src/models/mod.rs`；第二，文件建错位置（放到 `src/models/mod.rs` 之外的目录）一样报 E0583——**目录结构必须和模块树同名**；第三，同时存在 `src/models.rs` 和 `src/models/mod.rs` 会让编译器不知道用哪个，只保留一个。注意 `pub struct` 的字段仍默认私有，所以这里每个字段都写了 `pub`，理由在 D44。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 文件放哪，模块就在哪 | 模块树由 `mod` 声明决定，文件只是"内容放哪"的映射 |
| 拆文件会改变代码行为 | 只是把同一份代码换个命名空间，编译产物不变 |
| 必须先建好目录再写 `mod` | 顺序无所谓，但 `mod models;` 要求 `src/models.rs` 必须存在 |
| 拆成文件后就不用 `mod` 了 | 恰恰相反：每加一个文件就要在**它的父模块**里加一行 `mod` |

**在 tmdb-organizer 里**：第 7 周结束后项目变成 `main.rs` + `models.rs` + `parser.rs` 三件套；第 15 周会沿同一条路子继续拆出 `cli.rs`、`tmdb.rs`、`plan.rs`、`rename.rs`，而最终的目录结构在 README 末尾已经画好——**今天是从"一个文件"到"一棵模块树"的第一步。**

### D44 · 模块定义与 `pub` 可见性

📖 [书 §7.2](https://kaisery.github.io/trpl-zh-cn/ch07-02-defining-modules-to-control-scope-and-privacy.html)

**`pub` 是"向上开放一层"的开关。** Rust 默认**一切私有**：`struct` 私有、字段私有、函数私有。私有的含义是"只有本模块及其子模块能用"——所以父模块（`main.rs`）反而**看不到**子模块的私有项。写 `pub` 就是把可见性放开给父模块、祖父模块，一直到 crate 外。这不是为了折磨人，而是让你**主动决定哪些东西是 API**：`pub fn parse_filename` 是接口，内部辅助函数保持私有，将来改它就不会波及外部。

**结构体私有和字段私有是两把锁，要分别开。** `pub struct AnimeFile` 只让"这个名字"从外面可见，里面的 `name` 字段仍是私有的；`main.rs` 里构造它会报 **field `name` of struct `AnimeFile` is private**（E0451）。想让外部构造，就得给字段也加 `pub`——**或者**一个 `pub` 都不加、改用 `AnimeFile::new()` 当受控构造器（D35 的思路）。后者更安全：**字段私有 + 公开构造器 = "只能按我的规则造"**，D43 的 `models.rs` 选了前者，是因为这个阶段字段本身就是公开数据。

**为什么解析层要把错误类型一起 `pub`。** `ParseError` 是 `parse_filename` 签名的一部分，调用方要能把它 `match` 开才能区分失败原因（缺季、缺集、数字非法）。D28 的结论在这里落地：**错误类型属于模块的 API，不是内部细节**——所以它的变体也要 `pub`（enum 的变体默认继承 enum 的可见性）。

src/parser.rs
```rust
// 解析层：自带错误类型，From 让 ? 自动把 ParseIntError 转成 ParseError
use std::num::ParseIntError;

#[derive(Debug, PartialEq)]
pub enum ParseError {
    MissingSeason,
    MissingEpisode,
    InvalidNumber(ParseIntError),
}

impl From<ParseIntError> for ParseError {
    fn from(e: ParseIntError) -> Self {
        ParseError::InvalidNumber(e)
    }
}

pub fn parse_filename(name: &str) -> Result<(u32, u32), ParseError> {
    let s = name.find('S').ok_or(ParseError::MissingSeason)?;
    let e = name.find('E').ok_or(ParseError::MissingEpisode)?;
    let season: u32 = name[s + 1..e].parse()?;
    let tail = &name[e + 1..];
    let cut = tail.find('.').unwrap_or(tail.len());
    let episode: u32 = tail[..cut].parse()?;
    Ok((season, episode))
}
```

⚠️ 三个坑：第一，模块里的 `struct` 没写 `pub`，外面用就报 **struct `AnimeFile` is private**（E0603）——注意是**引入名字**时就报错，不是构造时；第二，结构体 `pub` 了但字段没 `pub`，构造时报 **field `name` of struct `AnimeFile` is private**（E0451），报错会指出"字段定义在这里"，但根因在可见性；第三，`pub` 不能"穿透"——`pub` 的函数返回私有类型会报 **type `ParseError` is private**，公开的东西必须由公开的东西组成。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 只有 `main.rs` 能随便用别的模块 | 子模块对父模块私有，`main.rs` 只能用子模块 `pub` 出来的东西 |
| `pub` 加在模块上就够了 | 每一项（struct / 字段 / fn / enum 变体）各有自己的可见性 |
| `pub` 只是给编译器看的修辞 | 它定义了 crate 的 API 边界，别人写代码只能依赖这些 `pub` 项 |
| 私有函数没法测 | 同文件里的 `#[cfg(test)] mod tests` 是子模块，照样能测（D46） |

**在 tmdb-organizer 里**：`src/parser.rs` 里 `pub fn parse_filename` 与 `pub enum ParseError` 是这一天的产出；`src/models.rs` 则把 `AnimeFile` / `Episode` 的字段全开成 `pub`（它们只是数据容器）。到第 15 周加 `AnimeFile::new()` 时，才会把字段收回来变私有——**"什么时候开、什么时候关"就此定了型。**

### D45 · 路径与 `use` 引入

📖 [书 §7.3](https://kaisery.github.io/trpl-zh-cn/ch07-03-paths-for-referring-to-an-item-in-the-module-tree.html)、[书 §7.4](https://kaisery.github.io/trpl-zh-cn/ch07-04-bringing-paths-into-scope-with-the-use-keyword.html)

**路径的起点有三种写法。** `crate::` 从当前 crate 根开始（`main.rs` 所在的位置），`self::` 从当前模块开始，`super::` 从父模块开始——`super::` 最常用来在子模块里回头引用父模块的东西。**选哪种取决于你在哪个模块里写**：`use` 的路径起点永远是 crate 根而不是当前文件所在目录，所以在 `models.rs` 里想引用解析模块必须写 `crate::parser::parse_filename`，写成 `parser::parse_filename` 会报 **unresolved import `parser`**（E0432），并提示 **a similar path exists: `crate::parser`**。

**`use` 只是给路径起个短名，不改变所有权、也不影响编译产物。** `use crate::parser::parse_filename;` 之后就能写 `parse_filename(...)`。它是**编译期别名**，运行时零开销。这条性质解释了两个常见习惯：模块本身常用 `use std::fmt;` 再写 `fmt::Result`（保留"这东西来自哪"的信息），而类型通常直接引到名字（因为类型名本身信息量足够）。社区经验是——**函数引到名字，模块保留一层**，因为 `parser::parse_filename` 比 `parse_filename` 更容易定位。

**trait 有个特例：不 `use` 就不认方法。** 这一条最容易绊人（D38 埋的伏笔）。`EpisodeFormatter` 已经在 `impl` 里生效，但只要它不在当前作用域，`f.label()` 就报 **no method named `label` found for struct `AnimeFile`**（E0599），并提示 **items from traits can only be used if the trait is in scope**——**trait 必须被引入作用域，它的方法才可用。**

src/main.rs
```rust
mod models; // 声明子模块：编译器会去找 src/models.rs
mod parser; // 同理去找 src/parser.rs

use crate::models::{AnimeFile, Episode}; // 类型直接引入，后面用短名
use crate::parser::parse_filename; // 函数也可以只引入名字

fn main() {
    for name in ["Bleach.S01E366.mkv", "Bleach.episode.mkv"] {
        match parse_filename(name) {
            Ok((season, episode)) => {
                let f = AnimeFile { name: String::from("Bleach"), season, episode };
                let ep = Episode { season, episode, title: format!("第 {episode} 集") };
                println!("{} / {}", f.label(), ep.tag());
            }
            Err(err) => println!("{name} 跳过：{err:?}"),
        }
    }
    // Bleach S01E366 / S01E366 第 366 集
    // Bleach.episode.mkv 跳过：MissingSeason
}
```

⚠️ 三个坑：第一，把同一层里两个同名项都 `use` 进来，报 **the name `Episode` is defined multiple times**（E0252），可以用 `as` 起别名（`use crate::models::Episode as LocalEpisode;`）；第二，忘记 `use` trait 却调它的方法，报 **no method named `label` found for struct `AnimeFile`**（E0599），提示 **items from traits can only be used if the trait is in scope**；第三，在子模块里用相对路径引用别的模块，报 **unresolved import**（E0432）——`use` 路径的起点是 crate 根，不是当前文件所在目录。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `use` 会把代码复制到当前文件 | 只是编译期别名，不产生任何运行时代码 |
| 不 `use` trait 也能调它的方法 | 不行，方法解析要求 trait 在作用域内，否则报 E0599 |
| `use` 的路径从当前文件所在目录算起 | 从 crate 根算起，所以跨模块一律 `crate::` 起头 |
| `use` 和 `mod` 是一回事 | `mod` 是"把文件接进模块树"，`use` 只是"起短名"，必须先用 `mod` 声明 |

**在 tmdb-organizer 里**：这三行 `mod` / `use` 就是第 7 周之后 `main.rs` 的开头，此后所有周（CLI、HTTP、计划生成）都在这套骨架里加模块。第 15 周 `main.rs` 会瘦成"`mod` 一堆 + 几行 `use` + 一个 `main`"——**`use` 写得好不好，直接决定 `main` 能不能一眼读完。**

### D46 · 单元测试与 `#[cfg(test)]`

📖 [书 §11.1](https://kaisery.github.io/trpl-zh-cn/ch11-01-writing-tests.html)

**`#[cfg(test)]` 让测试代码"只在 `cargo test` 时存在"。** `cfg` 是条件编译：`cargo build` 时不定义 `test`，整块 `mod tests` 被**直接丢弃**，产物里一行都没有；`cargo test` 会额外编一个带 `test` 配置的版本。这不是性能优化，而是关键保障——**测试可以随便调私有函数、随便构造非法数据，因为它们永远不会被发布出去。**

**为什么单元测试要写在模块内部。** `mod tests` 放在 `parser.rs` 里，它就成了 `parser` 的**子模块**，而 Rust 的可见性规则是"子模块能看父模块的一切"（包括私有项）。所以 `use super::*;` 之后，连私有的辅助函数都能被断言。这也划出了它和 D47 集成测试的分界：**单元测试测"内部实现细节"，集成测试只测"对外 API"。**

**每个 `#[test]` 是独立线程，一个失败不影响其他。** `cargo test` 默认并发跑用例，某个用例 panic 只会让它自己变红；`assert_eq!` 失败时会同时打印左右两边的**实际值**，比 `assert!(a == b)` 好用得多——这是 D28 手写断言的正式版。还有一个前提别忘：`assert_eq!` 要求两边都能比较和打印，所以 `ParseError` 必须有 **`#[derive(Debug, PartialEq)]`**，少一个都会报错。

src/parser.rs（在文件末尾追加）
```rust
#[cfg(test)]
mod tests {
    use super::*; // 引入父模块的一切，包括私有项

    #[test]
    fn parses_standard_name() {
        assert_eq!(parse_filename("Bleach.S01E366.mkv"), Ok((1, 366)));
    }

    #[test]
    fn reports_missing_season() {
        assert_eq!(parse_filename("Bleach.mkv"), Err(ParseError::MissingSeason));
    }
}
```

⚠️ 三个坑：第一，`assert_eq!` 比 `Result` 时忘了给 `ParseError` 派生 `PartialEq`，报 **binary operation `==` cannot be applied to type `Result<_, ParseError>`**（E0369），并提示 **an implementation of `PartialEq` might be missing for `ParseError`**；第二，`mod tests` 里不写 `use super::*;`，调 `parse_filename` 报 **cannot find function `parse_filename` in this scope**（E0425）——**子模块不会自动看到父模块的名字，得显式引入**；第三，把 `#[cfg(test)]` 写在 `mod tests` 之外（比如只写 `#[test]`），`cargo build` 时也会去编测试代码，一旦引用了只在 dev 依赖里有的东西就直接编译失败。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 测试代码会进最终二进制 | `#[cfg(test)]` 的块在普通构建里被整块丢弃，零残留 |
| 单元测试只能测 `pub` 函数 | 同文件的子模块能测私有项，这正是它和集成测试的分工 |
| `assert!` 和 `assert_eq!` 差不多 | `assert_eq!` 失败会打印两个实际值，定位问题快得多 |
| 一个用例 panic 会让整轮测试中断 | 每个用例独立线程，只有它自己变红，其余照跑 |

**在 tmdb-organizer 里**：`src/parser.rs` 末尾的 `#[cfg(test)] mod tests` 是这一天的产出，第 8 周给 `scan_dir` 加过滤逻辑时，同样的写法会出现在新的模块里。**规矩定下来：每加一个 `pub fn`，就在同一个文件里补两条用例。**

### D47 · 集成测试的组织结构

📖 [书 §11.3](https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html)

**集成测试和单元测试的差别，本质是"链接的是哪个 crate"。** `tests/` 下**每个 `.rs` 文件都被编译成一个独立的 crate**，它们只能使用**库 crate 的公开 API**，看不到任何私有项，也无法访问二进制 crate 内部的模块。这是设计而不是限制：它强制你从"外部使用者"的视角写测试，**只依赖 `pub` 的东西**——如果测试很难写，往往说明 API 设计得不好。

**所以两者的分工是清楚的。** 单元测试和源码在同一个文件里，能钻进 `parse_filename` 的每个内部细节；集成测试只摆"用户会怎么用"的用例，比如"给定一个《死神》文件名，能解析出季和集"。前者改了内部实现就会碎，后者只在**行为**变了时才碎——**集成测试是重构时的安全网，单元测试是开发时的放大镜。**

**为什么这里要写 `#[path]`。** 本项目目前是**纯二进制 crate**（没有 `src/lib.rs`），`use tmdb_organizer::parser::parse_filename;` 会直接报 **failed to resolve: use of unresolved module or unlinked crate**（E0433）。集成测试只能链接**库** crate，所以这里用 `#[path = "../src/parser.rs"] mod parser;` 把源文件直接编进测试 crate。等第 15 周把逻辑挪进 `src/lib.rs`，就能改回标准的 `use` 写法——**这也是"为什么该有库 crate"的第一次真实需求。**

tests/parser_test.rs
```rust
// 集成测试：tests/ 下每个文件都是独立 crate，只该用"外部使用者"的视角测
#[path = "../src/parser.rs"] // 本项目还没有 lib crate，先把解析模块直接编进来
mod parser;

use parser::{parse_filename, ParseError};

#[test]
fn parses_standard_name() {
    assert_eq!(parse_filename("Bleach.S01E366.mkv"), Ok((1, 366)));
}

#[test]
fn parses_with_resolution_suffix() {
    assert_eq!(parse_filename("Bleach.OVA.S01E02.1080p.mkv"), Ok((1, 2)));
}

#[test]
fn reports_garbled_numbers() {
    let result = parse_filename("Bleach.SxxEyy.mkv");
    assert!(matches!(result, Err(ParseError::InvalidNumber(_))));
}
```

⚠️ 三个坑：第一，直接 `use tmdb_organizer::...` 报 **failed to resolve: use of unresolved module or unlinked crate**（E0433），提示 **if you wanted to use a crate named `tmdb_organizer`, use `cargo add tmdb_organizer`**——**根因是没有 lib target，不是少装依赖**；第二，想在集成测试里调私有函数，报 **function `helper` is private**（E0603）；第三，`#[path]` 引入的模块自带的 `#[cfg(test)] mod tests` 也会被编进这个测试二进制，于是那两条单元用例在这里**又跑一遍**（名字带 `parser::tests::` 前缀）——`cargo test` 里同名用例出现两次不是重复造数据，就是这个原因。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `tests/` 里的文件会互相看见 | 每个文件是独立 crate，互不可见，也无法共享私有辅助函数 |
| 集成测试能测私有函数 | 只能测公开 API，测私有项要用单元测试 |
| 集成测试需要手动加进 `Cargo.toml` | 放在 `tests/` 下自动被发现，无需任何声明 |
| `tests/` 里的代码会影响发布产物 | 它只在 `cargo test` 时编译，不会进最终二进制 |

**在 tmdb-organizer 里**：`tests/parser_test.rs` 是这一天的产出，第 12 周加了 `parse_order` 后会在同一文件里补一组"未知顺序返回 `Err`"的用例；第 8 周的 `scan_dir` 也会照这个模式加一个集成测试文件。**注意第 15 周拆出 `src/lib.rs` 后，要把 `#[path]` 那行删掉、改回 `use tmdb_organizer::parser::...`。**

### D48 · `should_panic` 与测试筛选

📖 [书 §11.2](https://kaisery.github.io/trpl-zh-cn/ch11-02-running-tests.html)

**`#[should_panic]` 把"应该崩"变成一条断言。** 有些函数的契约就是"输入非法直接 panic"（比如 `expect`），这类行为也该被钉住。`#[should_panic]` 让用例**只有 panic 了才算通过**；加上 `expected = "..."` 还能要求 panic 消息包含某段文字，避免"因为别的原因崩了"也蒙混过关。**注意它断言的是"会崩"，所以别在同一个用例里既断言返回值又指望它 panic。**

**`cargo test` 的参数是"子串筛选"。** `cargo test parses` 只跑名字里含 `parses` 的用例，其余报告为 filtered out；要精确匹配用 `cargo test -- --exact parses_standard_name`。**`--` 是分界线**：前面给 cargo，后面给测试二进制本身——`--nocapture`（放开 `println!`）、`--ignored`、`--test-threads=1` 都是**测试二进制**的参数，写在 `--` 前面会报 **unexpected argument '--nocapture' found**，并提示 **to pass '--nocapture' as a value, use '-- --nocapture'**。

**测试是并发的，输出顺序不保证。** 每个用例一个线程，所以 `println!` 默认被捕获、只在失败时回放；也只有这样，一个用例 panic 才不会拖垮其他用例。一个直接推论：**别写依赖顺序的测试**——两个用例共用一个临时文件时，谁先跑完全看运气，这正是显式用 `--test-threads=1` 串行排障的场合。

tests/parser_test.rs（追加）
```rust
// should_panic：只断言"这里必须 panic"，且 panic 消息要含指定文字
fn expect_episode(name: &str) -> u32 {
    parse_filename(name).expect("文件名必须含 SxxExx").1
}

#[test]
#[should_panic(expected = "文件名必须含")]
fn panics_on_unparsable_name() {
    expect_episode("Bleach.mkv");
}

#[test]
fn filters_by_name() {
    // cargo test parses 时不会跑这个：名字里没有 parses
    assert_eq!(expect_episode("Bleach.S01E01.mkv"), 1);
}
```

⚠️ 三个坑：第一，`#[should_panic]` 的用例其实没 panic，报告 **test did not panic as expected**；第二，panic 消息与 `expected` 对不上，报告 **panic did not contain expected string** 并同时打印**panic message**与 expected substring 两个值——这条最有用，能区分"崩错了地方"；第三，`cargo test --nocapture` 报 **unexpected argument '--nocapture' found**，正确的写法是 `cargo test -- --nocapture`。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `#[should_panic]` 会检查 panic 的类型或位置 | 只检查"确实 panic 了"，加 `expected` 才检查消息子串 |
| `--nocapture` 放在 cargo 那一侧 | 必须放 `--` 之后，因为它是测试二进制自己的参数 |
| 用例按源码顺序执行 | 默认多线程并发，顺序不保证；要定序用 `--test-threads=1` |
| `cargo test` 参数和 `cargo run` 一样 | `cargo test` 支持名字筛选，`--` 之后才是测试二进制的参数 |

**在 tmdb-organizer 里**：`tests/parser_test.rs` 补上这三类边界用例（缺季、缺集、乱码 + panic 契约）后，第 12 周给集数映射加边界、第 13 周改 `build_plan` 拼名逻辑时，都靠这一批用例兜底。**"每加一条业务规则就补一条边界用例"从此成为习惯。**

### D49 · 文档注释与 `cargo fmt`

📖 [书 §14.2](https://kaisery.github.io/trpl-zh-cn/ch14-02-publishing-to-crates-io.html)、[附录 D](https://kaisery.github.io/trpl-zh-cn/appendix-04-useful-development-tools.html)

**`///` 不是普通注释，是"能被编译器读取的文档"。** `//` 写完就丢，`///` 会被收进 `cargo doc` 生成的 HTML，也会出现在编辑器的悬停提示里。里面可以直接写 Markdown（`# 标题`、`- 列表`），还有几个约定小节：`# Examples`、`# Panics`、`# Errors`。**`# Errors` 的价值在于把 D44 的错误类型写进 API 契约**——调用方看文档就知道该处理哪几种失败，而不是去翻源码。

**文档测试只在"库 crate"上跑。** 现在项目还没有 `src/lib.rs`，`cargo test --doc` 会报 **no library targets found in package `tmdb-organizer`**，于是 `# Examples` 里的代码**暂时不会被编译**。这一点要说清：现阶段 `///` 的收益是**编辑器悬停 + `cargo doc`**；等第 15 周拆出 `lib.rs`，那些示例才会自动变成可执行测试——这就是"文档即测试"的机制，也是**示例宁可写能跑的真代码**的理由。

**`cargo fmt` 是"让 diff 只包含意图"。** 它按官方风格（rustfmt）重排缩进、换行、逗号，把格式争论从 review 里剔掉。它**只改格式、不改语义**，但确实会改动代码：像前几周那种 `struct AnimeFile { name: String, season: u32, episode: u32 }` 的单行写法会被展开成多行。所以规矩是——先 `cargo fmt --check` 看差异，再 `cargo fmt` 落地，最后 `cargo test` 确认没改坏。

src/parser.rs
```rust
/// 从《死神》剧集文件名里解析出（季，集）。
///
/// # Examples
///
/// ```
/// use tmdb_organizer::parser::parse_filename;
/// assert_eq!(parse_filename("Bleach.S01E366.mkv"), Ok((1, 366)));
/// ```
///
/// # Errors
///
/// 缺季号返回 `ParseError::MissingSeason`，缺集号返回 `ParseError::MissingEpisode`，
/// 季或集不是合法数字时返回 `ParseError::InvalidNumber`。
pub fn parse_filename(name: &str) -> Result<(u32, u32), ParseError> {
    let s = name.find('S').ok_or(ParseError::MissingSeason)?;
    let e = name.find('E').ok_or(ParseError::MissingEpisode)?;
    let season: u32 = name[s + 1..e].parse()?;
    let tail = &name[e + 1..];
    let cut = tail.find('.').unwrap_or(tail.len());
    let episode: u32 = tail[..cut].parse()?;
    Ok((season, episode))
}
```

⚠️ 三个坑：第一，`cargo test --doc` 在纯二进制项目上直接报 **no library targets found in package `tmdb-organizer`**——不是示例写错了，而是没有库 target；第二，`///` 里写了围栏代码块但内容编译不过，将来挪进库 crate 后会变成**失败的 doctest**，所以现在就得保证示例是真的；第三，另一种写法 `/** ... */` 也能当文档注释，但 `///` 配合 Markdown 更常见，**别把 `//!` 和 `///` 弄混**——`//!` 是给所在模块/文件写文档（放在文件顶部），`///` 是给紧跟其后的那一项写文档。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `///` 和 `//` 只是写法不同 | `///` 进文档与悬停提示，`//` 只是给自己看的注释 |
| 文档注释里的示例总会被执行 | 只有库 target 才收集 doctest；二进制项目要靠 `lib.rs` 才行 |
| `cargo fmt` 会顺手改逻辑 | 只动排版与换行，不改语义；跑完仍要 `cargo test` 复核 |
| `//!` 和 `///` 可以互换 | `//!` 描述所在模块，`///` 描述紧随其后的项，位置错了就没文档 |

**在 tmdb-organizer 里**：`src/parser.rs` 的 `parse_filename` 是第一个写全 `# Examples` / `# Errors` 的公共函数；第 8 周的 `scan_dir`、第 13 周的 `build_plan` 都会照这个模板写。**第 7 周到此收官——项目有了模块边界、测试网和文档，接下来才敢真正碰文件系统。**

---

<a id="w8"></a>

## 第 8 周：文件系统，扫描本地目录

| 天   | 学 6 min                                    | 写 10 min                                                    | 验收 / commit             |
| ---- | ------------------------------------------- | ------------------------------------------------------------ | ------------------------- |
| D50  | `Path` / `PathBuf` 路径操作（[std](https://rustwiki.org/zh-CN/std/path/struct.Path.html)） | `fn scan_dir(dir: &Path) -> Result<Vec<PathBuf>, io::Error>` | 返回路径列表，`day50`     |
| D51  | `fs::read_dir` 遍历目录（[std](https://rustwiki.org/zh-CN/std/fs/fn.read_dir.html)） | 遍历目录，打印文件名                                         | `cargo run -- .`，`day51` |
| D52  | `Path::extension` 提取扩展名（[std](https://rustwiki.org/zh-CN/std/path/struct.Path.html#method.extension)） | 只保留 `.mkv`、`.mp4`                                        | 过滤成功，`day52`         |
| D53  | `fs::metadata` 读元数据（[std](https://rustwiki.org/zh-CN/std/fs/fn.metadata.html)） | 打印文件大小                                                 | 输出 bytes，`day53`       |
| D54  | `PathBuf` 转自定义结构体                    | 把路径转成 `AnimeFile`，解析失败就跳过                       | 生成列表，`day54`         |
| D55  | `tempfile` 临时目录测试（[docs](https://docs.rs/tempfile/latest/tempfile/)） | `cargo add tempfile --dev`，写扫描测试                       | `cargo test`，`day55`     |
| D56  | 复盘：真实目录的边界情况                    | 扫描你本地真实《死神》目录                                   | 打印结果，`day56`         |

### D50 · `Path` / `PathBuf` 路径操作

📖 [std](https://rustwiki.org/zh-CN/std/path/struct.Path.html)

**为什么不直接用 `String` 装路径。** 路径在 Windows 上用反斜杠、在类 Unix 上用斜杠，而且 Unix 上的文件名是**任意字节**、不保证是 UTF‑8。`String` 两样都保证不了：它既不会替你按平台规则处理分隔符，也可能根本装不下某个文件名。所以标准库把"路径"单独做成两个类型——`Path`（借用，像 `&str`）与 `PathBuf`（拥有、可增长，像 `String`）。用类型把"路径"和"普通文本"分开，后面 `join`、`rename` 时编译器才能替你按平台规则处理。

**`Path` 与 `PathBuf` 的分工和 `&str` / `String` 一一对应。** 函数参数收 `&Path`（只借看，不接管所有权），返回值用 `PathBuf`（新造一份给调用方）。两者互转成本极低：`PathBuf` 用 `.as_path()` 得到 `&Path`，`&Path` 用 `.to_path_buf()` 得到拥有型 `PathBuf`——`to_path_buf` 只复制**路径字符串**，不碰磁盘。

**拼路径要用 `join` / `push`，不是字符串加法。** `join` 会按当前平台插入正确分隔符，还有个常被忽略的规则：**被拼接的部分若是绝对路径，会整体替换掉前面的部分**（`Path::new("a").join("/etc/x")` 结果是 `/etc/x`）。`push` 是 `join` 的就地版本。打印路径必须走 `.display()`——`Path` 故意不实现 `Display`，因为它可能不是合法 UTF‑8，直接打印会有歧义。

```rust
use std::io;
use std::path::{Path, PathBuf};

fn scan_dir(dir: &Path) -> Result<Vec<PathBuf>, io::Error> {
    // 目前只演示路径拼装，D51 才真的去读目录
    Ok(vec![dir.join("Bleach.S01E01.mkv")])
}

fn main() {
    let dir = Path::new("anime");
    let files = scan_dir(dir).unwrap();
    println!("{}", files[0].display()); // anime\Bleach.S01E01.mkv（Windows；Linux/macOS 为 anime/Bleach.S01E01.mkv）
    println!("{:?}", files[0].file_name()); // Some("Bleach.S01E01.mkv")
    println!("{:?}", files[0].extension()); // Some("mkv")
    println!("{}", files[0].parent().unwrap().display()); // anime

    let mut p = PathBuf::from("anime");
    p.push("S01");
    p.push("Bleach.S01E01.mkv");
    println!("{}", p.display()); // anime\S01\Bleach.S01E01.mkv
}
```

⚠️ 三个坑：第一，直接 `println!("{}", path)` 打印 `PathBuf`，报 **PathBuf doesn't implement std::fmt::Display**，路径要么 `.display()`、要么用 `{:?}`；第二，想用 `+` 拼路径，报 **cannot add &str to PathBuf**，正确做法是 `join`/`push`；第三，`PathBuf` 没实现 `Copy`，`let a = PathBuf::from("x"); let b = a;` 之后再用 `a` 会报 **borrow of moved value**——注意 `let files = scan_dir(dir)` 本身没事，因为传进去的只是 `&Path`。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `Path` 可以像 `String` 一样 `println!` | 不实现 `Display`，得 `.display()`，或用 `{:?}` |
| `Path` 和 `PathBuf` 是同一个东西 | 前者像 `&str`（借用），后者像 `String`（拥有） |
| `join` 只是把字符串接起来 | 会按平台插入分隔符，且绝对路径会覆盖前面的部分 |
| 路径可以用 `String` 代替 | 非 UTF‑8 文件名 `String` 装不下，这正是路径要独立类型的原因 |

**在 tmdb-organizer 里**：`scan_dir(dir: &Path) -> Result<Vec<PathBuf>, io::Error>` 就是这一天的产出签名；第 14 周 `fs::rename(&from, &to)` 会直接吃这些 `PathBuf`，中间不需要再转回字符串。

### D51 · `fs::read_dir` 遍历目录

📖 [std](https://rustwiki.org/zh-CN/std/fs/fn.read_dir.html)

**`read_dir` 返回的是"两层 Result"。** 外层 `Result<ReadDir, io::Error>` 表示"这个目录能不能打开"（不存在、没权限就在这一层失败）；内层的迭代器产出 `Result<DirEntry, io::Error>`，因为**目录内容在遍历途中可能变化**——文件被删、权限被撤，都会让单个条目失败。两层分开是刻意的：外层失败说明"整个扫描没意义"，内层失败只说明"跳过这一个，接着扫"。

**为什么内层错要"跳过"而不是终止。** 真实目录里总有读不出来的东西（损坏的符号链接、系统占位文件）。如果内层直接 `unwrap`，一个坏条目就能让整个工具崩掉；D13 定下的原则在这里兑现：**能继续就继续**，写法是把它 `.ok()` 之后丢掉 `None`。

**遍历是"只此一层"的。** `read_dir` 不会递归进子目录，这对整理《死神》正合适：剧集文件一般平铺在季文件夹里。`DirEntry::path()` 只是把目录路径和文件名拼起来，**不额外访问磁盘**；`file_name()` 给的是 `OsString`（因为不保证 UTF‑8），要文本得自己转。

```rust
use std::fs;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let dir = Path::new("anime");
    // read_dir 只遍历一层；每个条目本身是 Result，读不出来的要单独处理
    for entry in fs::read_dir(dir)? {
        let entry = entry?;                          // 拿到 DirEntry
        let path = entry.path();                     // 拼成完整 PathBuf
        println!("{:?}", path.file_name().unwrap()); // "Bleach.S01E01.mkv"
    }
    Ok(())
}
```

⚠️ 三条：第一，忘了 `let entry = entry?;` 就直接 `entry.path()`，报 **no method named path found for enum Result**，错误会提示你用 `?` 取出 `DirEntry`；第二，`println!("{}", entry.file_name())` 报 **OsString doesn't implement std::fmt::Display**，`OsString` 要用 `{:?}` 或 `.to_string_lossy()`；第三，在返回 `()` 的 `main` 里对 `fs::read_dir(dir)` 用 `?`，报 **the ? operator can only be used in a function that returns Result or Option**——要么把 `main` 改成 `-> std::io::Result<()>`，要么显式处理。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 迭代器每个元素就是 `DirEntry` | 是 `Result<DirEntry, io::Error>`，单个条目失败要单独决定"跳过还是中止" |
| `read_dir` 会递归进子目录 | 只遍历一层，要递归得自己在循环里判断 `is_dir` 再进 |
| 遍历顺序按文件名排好 | 顺序由文件系统决定、不保证；要稳定输出得自己 `sort` |
| 出错的条目会让整轮扫描失败 | 内层用 `.ok()` / `?` 分别处理，坏条目可跳过而不影响其余 |

**在 tmdb-organizer 里**：`scan_dir` 的核心就是这个 `for` 循环——D52 在循环体里加扩展名过滤，D53 加大小统计，D54 才把 `PathBuf` 转成 `AnimeFile`。

### D52 · `Path::extension` 提取扩展名

📖 [std](https://rustwiki.org/zh-CN/std/path/struct.Path.html#method.extension)

**扩展名的定义是"最后一个点之后的部分"，而且有些名字根本没有扩展名。** `extension()` 返回 `Option<&OsStr>`：`Bleach.S01E01.mkv` 给 `mkv`，`Bleach.OVA` 给 `OVA`（点后面那一整段都算），而 `.gitkeep` 这类**点开头的隐藏文件**给 `None`，`..` 也是 `None`。自己写 `split('.')` 很容易在这些边界上出错——标准库这层封装让你只关心"最后一段是什么"。

**返回 `Option<&OsStr>` 而不是 `String`。** 一是扩展名可能不存在（用 `Option` 表达），二是它**借用原路径、零分配**，过滤时不必为每个文件造字符串。代价是它可能是非 UTF‑8，所以比较时用 `eq_ignore_ascii_case`（`OsStr` 上的方法）而不是 `==` 直接比字符串。

**大小写必须自己处理。** 文件系统不保证大小写一致：Windows 不区分（`Bleach.MKV` 和 `Bleach.mkv` 是同一个文件），Linux 区分。想跨平台行为一致，就得显式做大小写不敏感比较；`eq_ignore_ascii_case` 只对 ASCII 做转换、不牵扯 Unicode 大小写，正好够用。

```rust
use std::path::Path;

fn main() {
    let names = [
        "Bleach.S01E01.mkv",
        "Bleach.S01E02.MKV", // 大小写不同，也要留下
        "notes.txt",         // 不是视频，过滤掉
        "Bleach.OVA",        // 扩展名是 "OVA"，一样被过滤
    ];
    let videos: Vec<&str> = names
        .iter()
        .filter(|name| {
            Path::new(name)
                .extension()
                .is_some_and(|ext| ext.eq_ignore_ascii_case("mkv") || ext.eq_ignore_ascii_case("mp4"))
        })
        .copied()
        .collect();
    println!("{:?}", videos);
    // ["Bleach.S01E01.mkv", "Bleach.S01E02.MKV"]
}
```

⚠️ 三条：第一，想当然地写 `path.extension() == Some("mkv")`，报 **mismatched types**（expected &OsStr, found &str），要么用 `OsStr::new("mkv")`，要么改用 `eq_ignore_ascii_case`；第二，把 `&OsStr` 当 `&str` 用（比如 `.to_lowercase()`），报 **no method named to_lowercase found for reference &OsStr**，得先 `.to_string_lossy()` 转成文本；第三，`.gitkeep` 这类点开头的文件**没有扩展名**，而 `Bleach.OVA` 的扩展名恰好是 `OVA`——别把"点后面没东西"和"点后面是文字"搞反。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 没有点就没有扩展名，有点就有 | 点开头的隐藏文件（`.gitkeep`）**没有**扩展名；`Bleach.OVA` 的扩展名是 `OVA` |
| 扩展名比较用 `==` 就行 | `&OsStr` 与 `&str` 不能直接比，得转或用 `eq_ignore_ascii_case` |
| `.mkv` 和 `.MKV` 一定都能匹配 | 取决于你比较的方式；不显式忽略大小写，`.MKV` 会被漏掉 |
| `extension()` 会为每个文件分配字符串 | 借用原路径、零分配；这是它返回 `&OsStr` 的原因 |

**在 tmdb-organizer 里**：`scan_dir` 循环体里的那个 `filter` 就是这一行；把 `mkv`、`mp4` 抽成一个常量数组，将来要支持 `avi` 只改一处。

### D53 · `fs::metadata` 读元数据

📖 [std](https://rustwiki.org/zh-CN/std/fs/fn.metadata.html)

**`metadata` 换来的是一个结构体，而不是字符串。** 它一次给全：`len()`（字节数）、`is_file()` / `is_dir()`、`modified()`（`SystemTime`）、权限位。为什么不拆成好几个函数？因为**取元数据是一次系统调用**，一次拿全比问五次省事，也避免"问完大小再问类型时文件已被替换"的不一致。

**它默认跟随符号链接。** `fs::metadata` 会**跟到链接指向的真实文件**上取信息；如果你要的是"链接本身"的信息（判断它是不是链接、指向哪），得用 `fs::symlink_metadata`。整理剧集时通常希望跟随链接（用户可能用链接把剧集目录挂进来），所以默认用 `metadata`。

**`len()` 的语义有两处要注意。** 一是它返回 `u64`，二是它对**目录**返回的不是"目录里所有文件的总大小"（Windows 上通常是 0），别拿它算目录体积。另外 `modified()` 返回 `io::Result<SystemTime>`，而 `SystemTime` 不实现 `Display`，要打印得先 `duration_since` 或直接 `{:?}`。

```rust
use std::fs;
use std::path::Path;

fn human_size(bytes: u64) -> String {
    let mb = bytes as f64 / 1024.0 / 1024.0;
    format!("{mb:.1} MB")
}

fn main() -> std::io::Result<()> {
    let p = Path::new("Bleach.S01E01.mkv");
    fs::write(p, vec![0u8; 2 * 1024 * 1024])?; // 造一个 2 MiB 的假剧集文件
    let meta = fs::metadata(p)?;               // 一次系统调用取回元数据
    println!("是文件：{}", meta.is_file()); // 是文件：true
    println!("是目录：{}", meta.is_dir()); // 是目录：false
    println!("字节数：{}", meta.len()); // 字节数：2097152
    println!("大小：{}", human_size(meta.len())); // 大小：2.0 MB
    fs::remove_file(p)?;
    Ok(())
}
```

⚠️ 三条：第一，忘了 `?` 直接对 `Result<Metadata, io::Error>` 调 `.len()`，报 **no method named len found for enum Result**；第二，`println!("{}", meta.modified()?)` 报 **SystemTime doesn't implement std::fmt::Display**，时间要用 `{:?}` 或先转成 `Duration`；第三，`len()` 是**字节**，人类可读单位要自己除，注意 `u64` 整除会截断，先 `as f64` 再除。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 目录的 `len()` 是目录里文件的总大小 | 不是，Windows 上通常为 0；要统计体积得自己累加 |
| `metadata` 只拿大小 | 一次调用同时拿到类型、大小、时间、权限 |
| `metadata` 对符号链接取链接本身的信息 | 它跟随链接；要链接本身的信息用 `symlink_metadata` |
| `SystemTime` 能直接 `println!` | 不实现 `Display`；用 `{:?}` 或 `duration_since` 换算 |

**在 tmdb-organizer 里**：`human_size` 会被第 13 周的 dry-run 计划表复用来打印"能省下多少空间"，`is_file()` 则用来把子目录挡在扫描结果之外。

### D54 · `PathBuf` 转自定义结构体

📖 [std](https://rustwiki.org/zh-CN/std/path/struct.Path.html)

**这一步的本质是把"无类型的路径"变成"有类型的数据"。** `scan_dir` 给出的 `Vec<PathBuf>` 只知道"有这么些文件"，只有解析出 `(season, episode)` 之后，它才变成能参与集数映射的 `AnimeFile`。`AnimeFile` 里除了解析出的字段，还要留一份 `path: PathBuf`——因为第 14 周真正改名时，你需要知道"这条数据对应磁盘上哪个文件"。

**失败要跳过，所以整条链用 `Option` 串起来。** `to_anime_file` 做三件事：取文件名、转成 `&str`（失败说明非 UTF‑8）、解析季集（失败说明不符合命名规则）。任何一步返回 `None`，整个转换就返回 `None`，调用方用 `filter_map` 把它们安静丢掉。**`?` 在这里同时表达了"可能没有"和"为什么没有"**，比层层嵌套 `if` 干净得多。

**借用解析、拥有存放。** 函数参数是 `&Path`（只借看），但存进结构体的 `path` 得是 `PathBuf`，所以用 `to_path_buf()` 造一份拥有型。这正对应 D8 的规则：**要留在结构体里、活过函数调用的数据，必须有自己的主人。**

```rust
use std::path::{Path, PathBuf};

#[derive(Debug)]
struct AnimeFile {
    name: String,
    season: u32,
    episode: u32,
    path: PathBuf,
}

// 占位解析器：项目里真正的实现在 src/parser.rs（D44 的 Result 版）
fn parse_filename(name: &str) -> Option<(u32, u32)> {
    let s = name.find('S')?;
    let e = name.find('E')?;
    let season = name[s + 1..e].parse().ok()?;
    let tail = &name[e + 1..];
    let cut = tail.find('.').unwrap_or(tail.len());
    Some((season, tail[..cut].parse().ok()?))
}

// 关键一步：借用 &Path 解析，成功才把 PathBuf move 进 AnimeFile
fn to_anime_file(path: &Path) -> Option<AnimeFile> {
    let name = path.file_name()?.to_str()?; // 文件名不是 UTF-8 就放弃
    let (season, episode) = parse_filename(name)?; // 解析不出集号也放弃
    Some(AnimeFile { name: String::from("Bleach"), season, episode, path: path.to_path_buf() })
}

fn main() {
    let candidates = vec![
        PathBuf::from("anime/Bleach.S01E01.mkv"),
        PathBuf::from("anime/Bleach.S01E02.mkv"),
        PathBuf::from("anime/sample.mkv"), // 解析不出集号，跳过
    ];
    // filter_map：成功的留下，失败的安静跳过
    let files: Vec<AnimeFile> = candidates.iter().filter_map(|p| to_anime_file(p)).collect();
    println!("扫描到 {} 个", files.len()); // 扫描到 2 个
    println!("{}", files[0].path.display()); // anime/Bleach.S01E01.mkv
    println!("{} S{:02}E{:02}", files[0].name, files[0].season, files[0].episode); // Bleach S01E01
}
```

⚠️ 三条：第一，忘了 `?` 把 `Option<&OsStr>` 直接传给收 `&str` 的函数，报 **mismatched types**（expected &str, found Option<&OsStr>）；第二，结构体字段是 `PathBuf` 却把借来的 `path` 直接存进去，报 **mismatched types**（expected PathBuf, found &Path），错误会提示你写 `path.to_path_buf()`；第三，`to_str()` 对非 UTF‑8 文件名返回 `None`，这不是 bug，而是标准库在提醒你"这串字节不是合法 UTF‑8"，真要容错就用 `to_string_lossy()`。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `to_str()` 总能拿到字符串 | 文件名可能是非 UTF‑8 字节，`to_str()` 返回 `None`，要容错用 `to_string_lossy()` |
| `to_path_buf()` 会复制磁盘上的文件 | 只复制路径这串字符，不碰磁盘内容 |
| `filter_map` 和 `map` 差不多 | `filter_map` 丢弃返回 `None` 的项，正好表达"解析失败就跳过" |
| `AnimeFile` 里存 `String` 还是 `PathBuf` 无所谓 | 存 `PathBuf` 才能在后续 `rename` 时直接用，不必再转一次 |

**在 tmdb-organizer 里**：`to_anime_file(&Path) -> Option<AnimeFile>` 是 `scan_dir` 与 `AnimeFile` 之间的桥；`AnimeFile` 在 D5 只有 `name`/`season`/`episode`，今天补上 `path: PathBuf`（字段仍是 `pub`，沿用 D43/D44 的可见性约定）。

### D55 · `tempfile` 临时目录测试

📖 [docs](https://docs.rs/tempfile/latest/tempfile/)

**测试不能依赖你本机的真实目录。** 如果测试直接去读 `./Bleach`，那么在没有这个目录的机器（CI、队友的电脑）上就会失败——测试的可靠性不该取决于外部环境。`tempfile` 提供一个**唯一命名**的临时目录：名字带随机串，不会和别人撞，而且 `TempDir` 一出作用域就被**自动删除**（靠 `Drop`，正是 D8 讲的机制）。

**为什么它是 dev 依赖。** `cargo add tempfile --dev` 写进 `Cargo.toml` 的 `[dev-dependencies]`。`cargo test` 时会编进去，`cargo run` / `cargo build --release` 时**根本不参与**——发布出去的二进制不含它。这就是 dev 依赖的意义：测试要用的工具，不该拖累产物。

**它让"目录相关函数"第一次变得可测。** `scan_dir` 之前只能靠肉眼看结果；有了临时目录，测试能**自己造出想要的目录布局**（两个视频、一个 txt），再断言"只扫出两个"。输入完全可控，失败时也能精确定位是哪一项没被正确过滤。

```rust
#[cfg(test)]
mod tests {
    use super::*; // 拿到 src/main.rs 里的 scan_dir 和 fs
    use tempfile::tempdir;

    #[test]
    fn scans_only_video_files() {
        let dir = tempdir().unwrap(); // 唯一临时目录，出作用域自动删除
        fs::write(dir.path().join("Bleach.S01E01.mkv"), "x").unwrap();
        fs::write(dir.path().join("Bleach.S01E02.MKV"), "x").unwrap();
        fs::write(dir.path().join("notes.txt"), "x").unwrap(); // 应被过滤

        let files = scan_dir(dir.path()).unwrap();
        assert_eq!(files.len(), 2); // 只留下两个视频
        assert!(files.iter().all(|p| p.extension().unwrap().eq_ignore_ascii_case("mkv")));
    }

    #[test]
    fn missing_dir_is_error() {
        let dir = tempdir().unwrap();
        let gone = dir.path().join("not-here"); // 不存在的子路径
        assert!(scan_dir(&gone).is_err());
    }
}
```

⚠️ 三条：第一，想在 `cargo run` 里用 `tempfile::tempdir()`，报 **failed to resolve: use of unresolved module or unlinked crate tempfile**（提示 use `cargo add tempfile`）——但真正的原因是它只是 **dev 依赖**，只能在测试里用；第二，只留下 `TempDir::path()` 而让 `TempDir` 提前被 drop，目录会被**立刻删除**，后续写入报 **No such file or directory (os error 2)**——必须让 `TempDir` 活到测试结束；第三，临时路径别写死名字，`tempdir()` 每次都不一样，**不要试图在两次测试之间复用同一路径**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `tempfile` 加进依赖就能在 `cargo run` 里用 | dev 依赖只在测试/示例里可见，产物里没有它 |
| 临时目录要自己记得删 | `TempDir` 离开作用域自动删，靠 `Drop` |
| 临时目录名固定、方便写死路径 | 每次带随机后缀，刻意避免并发测试互相踩 |
| 测试必须依赖真实文件才能跑 | 自己造目录即可，测试才能脱离环境、可重复 |

**在 tmdb-organizer 里**：这一天的测试写在 `src/main.rs` 的 `#[cfg(test)] mod tests` 里；到第 14 周做真实改名（`fs::rename`）时，同一套 `tempdir` 写法会升级成"造文件 → 跑计划 → 断言文件名变了"（见 D97）。

### D56 · 复盘：真实目录的边界情况

📖 [std](https://rustwiki.org/zh-CN/std/fs/fn.read_dir.html)

**真实目录里什么都有，扫描器必须"活着走完"。** 你的《死神》目录里不会只有规范的 `.mkv`：可能有 `Season 1` 子目录、`.txt` 说明、系统生成的隐藏文件、带中文或空格的名字、大小写混用的扩展名，甚至没有权限的条目。第 8 周所有练习，本质上都在回答同一个问题：**遇到不像剧集的东西，是跳过还是报错？** 答案已经定下——跳过（除非整个目录根本打不开）。

**"跳过"要能解释清楚。** `filter_map` 把失败项丢掉了，但 dry-run 时用户需要知道"为什么这个文件没被整理"。所以第 13 周会在 `RenamePlan` 里加 `PlanStatus::Missing` 之类的状态，让"跳过"变得可见——**静默跳过和记录原因，是两个层次的事**。

**两个最容易忘的边界。** 一是**子目录**：`read_dir` 不递归，`Season 1` 会被当成一个普通条目；只看扩展名时它会因为"没有扩展名"被过滤掉，但如果你在不判断 `is_dir` 的情况下去读它，就会出错。二是**顺序**：`read_dir` 不保证顺序，同一目录两次扫描的清单可能不同，要稳定输出就必须自己排序。

```rust
use std::fs;

fn main() -> std::io::Result<()> {
    let root = std::env::temp_dir().join("rust-d56-demo");
    let _ = fs::remove_dir_all(&root);
    fs::create_dir_all(root.join("Season 1"))?; // 子目录：read_dir 不会自动进去
    fs::write(root.join("Bleach.S01E01.mkv"), "x")?;
    fs::write(root.join("Bleach.S01E02.MKV"), "x")?;
    fs::write(root.join("说明.txt"), "x")?; // 非视频，过滤掉
    fs::write(root.join(".gitkeep"), "x")?; // 点文件：扩展名是 None

    let (mut videos, mut skipped) = (0, 0);
    for entry in fs::read_dir(&root)? {
        let path = entry?.path();
        if path.extension().is_some_and(|e| e.eq_ignore_ascii_case("mkv")) {
            videos += 1;
        } else {
            skipped += 1; // 目录、txt、点文件都算跳过
        }
    }
    println!("视频 {videos} 个，跳过 {skipped} 项"); // 视频 2 个，跳过 3 项
    fs::remove_dir_all(&root)?;
    Ok(())
}
```

⚠️ 三条：第一，把 `DirEntry` 当 `Path` 用（直接 `entry.extension()`），报 **no method named extension found for struct DirEntry**，要先 `entry.path()`；第二，目录的 `metadata().len()` 在 Windows 上通常是 0，别用它统计"目录占用空间"；第三，`read_dir` 顺序不确定，想让输出可对比（测试、diff）就得先 `.sort()`，否则同一目录两次运行结果可能都不一样。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 目录里只有视频文件 | 还有子目录、隐藏文件、说明文件；扫描器要能忽略它们 |
| 跳过的文件不用管 | dry-run 时应说明"为什么没整理"，第 13 周会用状态枚举补上 |
| 每次扫描结果顺序一致 | `read_dir` 顺序由文件系统决定；要稳定得自己排序 |
| 目录不存在就该直接 panic | 应返回 `Err` 交给调用方决定怎么报错（D55 的第二个测试钉住了这点） |

**在 tmdb-organizer 里**：今天拿真实的《死神》目录跑一遍 `scan_dir`，把"没被扫到"的文件逐个归类（子目录 / 非视频 / 命名不规范），这份清单就是第 13 周 `PlanStatus` 的雏形。**第 8 周到此收官——项目从"能解析一个名字"升级成"能读懂一个目录"，下一周给它一个命令行入口。**

---

<a id="w9"></a>

## 第 9 周：CLI 参数 clap

| 天   | 学 6 min                                  | 写 10 min                                                    | 验收 / commit                  |
| ---- | ----------------------------------------- | ------------------------------------------------------------ | ------------------------------ |
| D57  | clap derive 模式与 `Parser`（[docs](https://docs.rs/clap/latest/clap/_derive/index.html)） | `cargo add clap --features derive`，定义 `Args { dir: PathBuf }` | 编译通过，`day57`              |
| D58  | 位置参数与 `#[arg]`（[docs](https://docs.rs/clap/latest/clap/_derive/index.html#arg-attributes)） | 用 `Args` 替换硬编码路径                                     | `cargo run -- .`，`day58`      |
| D59  | 默认值与 `ValueEnum`（[docs](https://docs.rs/clap/latest/clap/_derive/index.html#valueenum-attributes)） | 加 `--order`，默认 `tvdb`，解析成 `EpisodeOrder`             | 参数生效，`day59`              |
| D60  | bool 开关参数与 `ArgAction`（[docs](https://docs.rs/clap/latest/clap/_derive/index.html#arg-attributes)） | 加 `--dry-run` 布尔参数                                      | 打印 dry-run，`day60`          |
| D61  | 计数参数与 `help` 文本（[docs](https://docs.rs/clap/latest/clap/_derive/index.html#doc-comments)） | 加 `--verbose` 或 `--top`                                    | 参数生效，`day61`              |
| D62  | README 结构与安装/用法/示例               | 写安装、用法、示例                                           | `--help` 正确，`day62`         |
| D63  | 复盘：参数组合的实际表现                  | 所有 CLI 参数跑一遍                                          | `cargo run -- --help`，`day63` |

### D57 · clap derive 模式与 `Parser`

📖 [docs](https://docs.rs/clap/latest/clap/_derive/index.html)

**手写参数解析的代价。** 不用库的话，你得自己 `std::env::args()` 取参数、判断 `--help`、把 `"1"` 转成 `u32`、决定缺参数时报什么错——这些逻辑又多又容易漏（尤其 `--help`、`--version`、错误退出码这些"约定俗成"的行为）。clap 把它们全部接管：**你只声明"有哪些参数、什么类型"，解析、校验、帮助、报错都由它生成。**

**derive 模式把结构体变成参数表。** `#[derive(Parser)]` 在编译期读你的结构体：每个字段是一个参数，字段类型决定它怎么解析（`PathBuf` 收路径、`u32` 收整数、`bool` 是开关）。运行时 `Args::parse()` 读进程参数、填好结构体，代码里直接用 `args.dir` 就行——**类型转换在库内部完成，你的代码里没有一行字符串解析。**

**为什么这比 builder 模式好。** clap 也能用 `Command::new("x").arg(...)` 一个个搭，但那样"参数表"和"结构体"是两份、容易对不上。derive 版是**单一事实来源**：字段即参数，加字段就加参数，删字段就删参数。

```rust
use clap::Parser;
use std::path::PathBuf;

/// 整理《死神》剧集文件名
#[derive(Parser, Debug)]
#[command(name = "tmdb-organizer", version, about = "按 TMDB 集数顺序整理剧集文件")]
struct Args {
    /// 要扫描的目录
    dir: PathBuf,
}

fn main() {
    let args = Args::parse();
    println!("扫描目录：{}", args.dir.display());
}
```

⚠️ 三条：第一，忘写 `use clap::Parser;`，`Args::parse()` 报 **no function or associated item named parse found for struct Args**，提示 trait Parser which provides parse is implemented but not in scope；第二，忘写 `#[derive(Parser)]` 却留着 `#[command(...)]`，报 **cannot find attribute `command` in this scope**，并提示 might be missing a `derive` attribute；第三，运行时漏了必填位置参数，clap 会打印 **error: the following required arguments were not provided: <DIR>** 并以非 0 退出——**这是运行期行为，不是编译错误**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `Args::parse()` 是 clap 生成的关联函数 | 它来自 `Parser` trait，所以必须 `use clap::Parser;` |
| derive 版功能比 builder 版少 | 两者等价，derive 展开成 builder 调用；只是写法不同 |
| `--help` / `--version` 要自己实现 | `Parser` 自动生成 `-h/--help` 与 `-V/--version` |
| 测试 CLI 必须真的起进程 | 用 `Args::try_parse_from([...])` 就能在进程内测（D62） |

**在 tmdb-organizer 里**：从今天起 `main` 的第一行是 `let args = Args::parse();`，第 1-8 周那些硬编码的 `Path::new("anime")` 全部换成 `args.dir`。

### D58 · 位置参数与 `#[arg]`

📖 [docs](https://docs.rs/clap/latest/clap/_derive/index.html#arg-attributes)

**"位置参数"就是没有名字的参数。** `tmdb-organizer ./Bleach` 里的 `./Bleach` 不靠 `--` 开头，只靠位置。在 derive 里，**字段不写 `long`/`short` 就是位置参数**，而且**多个位置参数按字段声明顺序绑定**——第一个字段吃第一个位置参数。这和你在终端里敲的顺序是同一个顺序，所以字段顺序一旦调换，语义就整体错位。

**类型决定了它是不是必填。** `dir: PathBuf` 是**必填**；`season: Option<u32>` 因为 `Option` 而**可选**。clap 读类型：`Option<T>` 天然可省、`T` 默认必填。这比手写"检查参数个数够不够"安全得多——**"可不可以省略"这件事由类型表达**，和 D13 用 `Option` 表达"可能没有值"是同一思路。

**`#[arg(...)]` 是给字段补细节的地方。** 常用的几个：`value_name`（帮助里占位符怎么显示，如 `<DIR>`）、`default_value`（默认值）、`long`/`short`（命名开关）、`help`（说明）。它们不改变字段类型，只改变"帮助里怎么写、命令行怎么给"。

```rust
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, about = "整理《死神》剧集文件名")]
struct Args {
    /// 要扫描的目录（位置参数，按声明顺序绑定）
    #[arg(value_name = "DIR")]
    dir: PathBuf,

    /// 只看某一季（可选，省略表示全部）
    #[arg(value_name = "SEASON")]
    season: Option<u32>,
}

fn main() {
    let args = Args::parse();
    println!("目录：{}", args.dir.display()); // 例：anime
    println!("季筛选：{:?}", args.season); // 例：Some(1)，省略时为 None
}
```

⚠️ 三条：第一，给位置参数误加 `#[arg(long)]`，它就成了 `--dir`，运行 `cargo run -- anime` 报 **unexpected argument 'anime' found**；第二，忘了 `use std::path::PathBuf;`，报 **cannot find type PathBuf in this scope**；第三，两个类型相同的位置参数（比如两个 `PathBuf`）顺序写反，**编译器不会报错**——类型系统只看类型，看不出"目录"和"备份目录"的语义差别，这类错误只能靠 `value_name` 和文档提醒自己。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 位置参数也要 `#[arg(long)]` 才算数 | 不写 `long`/`short` **才是**位置参数 |
| 位置参数顺序随便 | 严格按字段声明顺序绑定，调换会静默错位 |
| `Option<T>` 也要用户显式给 | `Option` 让参数可选；`T` 默认必填 |
| `value_name` 会影响解析 | 它只改帮助里的显示名，不参与解析 |

**在 tmdb-organizer 里**：`Args { dir: PathBuf, season: Option<u32> }` 让"整理整个目录"和"只整理某一季"共用一个入口；后续 `--order`、`--dry-run` 都会挂到这个结构体上。

### D59 · 默认值与 `ValueEnum`

📖 [docs](https://docs.rs/clap/latest/clap/_derive/index.html#valueenum-attributes)

**`--order` 的取值不该是自由文本。** 如果它是 `String`，用户敲 `--order abslute`（拼错）也能进来，错误要拖到很后面才炸。用 `ValueEnum` 把取值**约束成一个枚举**：clap 会校验、拒绝非法值，还会把合法取值列进 `--help`。这和 D7 的 `EpisodeOrder` 正好对上——**枚举是"多选一"，命令行参数也是"多选一"**。

**默认值让"最常用的情况"零配置。** `default_value_t = EpisodeOrder::Tvdb` 表示"不写 `--order` 就按 TVDB 顺序"。它要求类型实现 `Clone`（clap 要把默认值复制进结果），所以枚举要加 `#[derive(Clone)]`。有了它，`tmdb-organizer ./Bleach` 就能直接跑，不必每次写全参数。

**值名默认被转成 kebab-case。** 变体 `Absolute` 在命令行里是 `absolute`。这是 clap 的默认约定；要改就用 `#[value(name = "absolute")]` 显式指定。**记住：命令行上的取值名不等于 Rust 里的变体名**，写帮助文档时要按前者写。

```rust
use clap::{Parser, ValueEnum};

/// 整理《死神》剧集文件名
#[derive(Parser, Debug)]
#[command(version, about = "按指定集数顺序整理")]
struct Args {
    /// 要扫描的目录
    dir: std::path::PathBuf,

    /// 集数顺序，默认 tvdb
    #[arg(long, value_enum, default_value_t = EpisodeOrder::Tvdb)]
    order: EpisodeOrder,
}

#[derive(ValueEnum, Clone, Debug)]
enum EpisodeOrder {
    Tvdb,
    Dvd,
    Absolute,
}

fn main() {
    let args = Args::parse();
    println!("顺序：{:?}", args.order); // 省略 --order 时为 Tvdb
}
```

⚠️ 三条：第一，忘了 `value_enum`，clap 会退回去要求类型实现 `FromStr`/`Display`，报 **the trait bound EpisodeOrder: ToString is not satisfied**（help 里会说 Display 未实现）；第二，忘了 `#[derive(Clone)]`，报 **the trait bound EpisodeOrder: Clone is not satisfied**（`ValueEnum` 的约束就是 Sized + Clone）；第三，运行 `--order Absolute` 报 **invalid value 'Absolute' for '--order <ORDER>'** 并列出 **possible values: tvdb, dvd, absolute**——**取值是小写 kebab-case**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 命令行取值就是变体名原样 | 默认转 kebab-case：`Absolute` → `absolute` |
| 不写 `default_value` 也会用 `Default` | 不会，必须显式 `default_value_t` / `default_value` |
| 非法取值会被静默忽略 | clap 直接报错，并列出所有合法取值 |
| `value_enum` 只影响帮助显示 | 它同时接管校验与解析，是类型安全的一环 |

**在 tmdb-organizer 里**：`--order` 解析出的 `EpisodeOrder` 会喂给第 13 周的 `build_plan(local, remote, order)`，决定"按哪个顺序把本地文件映射到远程集数"——默认 `Tvdb` 覆盖大多数场景。

### D60 · bool 开关参数与 `ArgAction`

📖 [docs](https://docs.rs/clap/latest/clap/_derive/index.html#arg-attributes)

**bool 字段默认就是"开关"，不是"要值的参数"。** `#[arg(long)] dry_run: bool` 生成的是 `--dry-run`：出现即 `true`，不出现即 `false`，**不接受后面跟一个值**。这符合 CLI 的直觉（`--dry-run`、`--force` 都这样），也省掉了 `--dry-run true` 这种啰嗦写法。

**`ArgAction` 决定"参数被消费时的动作"。** 对 bool 默认是 `SetTrue`；还有 `SetFalse`（出现即置 `false`，适合 `--no-xxx` 之类的否定开关）、`Count`（重复计数，D61）、`Append`（重复收集）。所以 `no_overwrite: bool` 配上 `action = ArgAction::SetFalse` 时，**它的默认值是 `true`**——"不写这个标志"意味着"仍然保护已有文件"。

**为什么用动作属性而不是自己取反。** 你可以定义 `overwrite: bool` 再在代码里写 `!args.overwrite`，但那样**默认值语义被藏进了代码**；用 `SetFalse` 把"默认开启"写进参数表，`--help` 和代码读起来一致。**参数的默认行为属于参数定义，不该散落在 if 里。**

```rust
use clap::{ArgAction, Parser};

/// 整理《死神》剧集文件名
#[derive(Parser, Debug)]
#[command(version, about = "预览重命名计划")]
struct Args {
    /// 要扫描的目录
    dir: std::path::PathBuf,

    /// 只打印计划，不真的改文件名
    #[arg(long)] // bool 字段默认动作就是 SetTrue
    dry_run: bool,

    /// 关闭覆盖保护（默认开启）
    #[arg(long, action = ArgAction::SetFalse)]
    no_overwrite: bool,
}

fn main() {
    let args = Args::parse();
    println!("dry_run = {}", args.dry_run); // 省略时为 false
    println!("no_overwrite = {}", args.no_overwrite); // 省略时为 true
}
```

⚠️ 三条：第一，bool 字段忘了 `#[arg(long)]`，clap 会在启动时 panic：**Argument 'dry_run' is positional and it must take a value but action is SetTrue**——bool 不能当位置参数；第二，把开关当值参数写 `--dry-run true`，报 **unexpected argument 'true' found**；第三，`SetFalse` 的默认值是 `true`（`no_overwrite` 省略时即为 true），**和普通 bool 的直觉相反**，用之前先在心里过一遍"不写它会怎样"。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| bool 参数要写成 `--flag true` | 默认是开关（`SetTrue`），出现即真、不取值 |
| `ArgAction::SetFalse` 的默认是 false | 默认是 **true**，标志出现才变 false |
| 可以用 `--dry-run=false` | 默认不接受值；要能赋值得显式改成 `ArgAction::Set` |
| `ArgAction` 是运行时开关 | 它是编译期属性，决定 clap 如何消费该参数 |

**在 tmdb-organizer 里**：`--dry-run`（第 14 周真正接上 `exec_plan`）就是这一天的产出——`if args.dry_run { 打印计划 } else { 执行改名 }`，正是 D4 那条 `if/else` 的最终形态。

### D61 · 计数参数与 `help` 文本

📖 [docs](https://docs.rs/clap/latest/clap/_derive/index.html#doc-comments)

**"详细程度"是计数，不是开关。** 日志分级常有 `-v`、`-vv`、`-vvv` 这种递进，一个 bool 表达不了。`ArgAction::Count` 让参数每出现一次就加一，字段类型是整数（一般 `u8`）：省略是 0、`-vv` 是 2。**把"重复几次"编码成一个数，比定义 `--verbose1/--verbose2` 简洁**，也是 Unix 工具的通行做法。

**帮助文本来自文档注释。** clap 的 derive 会读 `///`：写在**字段上**的 `///` 成为该参数的 `help`，写在**结构体上**的 `///` 成为程序的 `about`。所以写帮助不必额外写 `help = "..."`，日常注释顺手就生成了帮助。**注意是 `///` 而不是 `//`**：`//` 对 clap 完全不可见，`--help` 里那一栏会是空的。

**`#[command(...)]` 管程序级信息。** `version` 让 `-V/--version` 打印版本（取自 `Cargo.toml`），`about` 是简介，`long_about = None` 表示"别再用文档注释补一段长说明"。这些属性和字段的 `#[arg]` 分工明确：**一个描述"程序是什么"，一个描述"参数是什么"。**

```rust
use clap::{ArgAction, Parser};

/// 整理《死神》剧集文件名
#[derive(Parser, Debug)]
#[command(version, about = "按 TMDB 顺序整理", long_about = None)]
struct Args {
    /// 要扫描的目录
    dir: std::path::PathBuf,

    /// 增加日志详细度，可重复：-v、-vv、-vvv
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,
}

fn main() {
    let args = Args::parse();
    println!("目录：{}，verbose = {}", args.dir.display(), args.verbose);
}
```

⚠️ 三条：第一，用了 `ArgAction` 却忘写 `use clap::ArgAction;`，报 **failed to resolve: use of undeclared type ArgAction**（提示 consider importing this enum）；第二，只写了 `long` 没写 `short`，`-v` 不可用，运行 `-vv` 报 **unexpected argument '-v' found**——想让 `-v` 生效必须同时给 `short`；第三，把 `Count` 用在 `bool` 字段上，clap 启动即 panic：**Argument 'verbose's selected action Count contradicts 'value_parser' (ValueParser::bool)**——**Count 必须和整数类型搭配**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 用 `//` 写的说明会进 `--help` | 只有 `///` 会被 clap 读取；`//` 完全不可见 |
| 字段上的注释和结构体上的一样 | 字段上的成为参数 `help`，结构体上的成为程序 `about` |
| `--verbose` 需要一个值 | `Count` 动作下它不取值，重复出现即累加 |
| `Count` 可以配 `bool` | 不行，运行时 panic；要配 `u8` 之类的整数 |

**在 tmdb-organizer 里**：第 15 周的 `--verbose`（D100 的日志分级）就是今天这个 `Count` 参数；而 `///` 写的帮助会让 `--help` 自带一份可读的用法说明，正好是 D62 写 README 的素材来源。

### D62 · README 结构与安装/用法/示例

📖 [docs](https://docs.rs/clap/latest/clap/_derive/index.html)

**README 的读者是"没有上下文的人"。** 他自己 clone 了仓库，只知道这是个"整理剧集"的工具。所以结构要按**他的阅读顺序**排：这个工具能做什么（一句话）→ 怎么装 → 怎么配（API key）→ 怎么用（命令示例）→ 一个完整例子 → 注意事项。**每一节都在回答"我下一步该敲什么"。**

**示例命令必须是"可复制的真命令"。** README 里写的 `tmdb-organizer ./Bleach --order absolute --dry-run` 如果参数名打错、或者根本解析不了，读者复制过去就报错，信任立刻归零。**所以示例最好有测试兜底**：用 `Args::try_parse_from` 把 README 里那条命令原样喂给解析器，能解析才算通过——**让文档变成可执行的契约**（这是 D49"文档即测试"的延伸）。

**`--help` 是 README 的种子。** clap 生成的 `--help` 已经把参数、默认值、可选值都列好了；README 的"用法"一节基本就是"把 `--help` 的输出翻译成人话"。写好每个字段的 `///`，README 和 `--help` 就都省事了。

```rust
#[cfg(test)]
mod tests {
    use super::*; // 拿到 src/main.rs 里的 Args 与 EpisodeOrder

    #[test]
    fn usage_example_in_readme_parses() {
        // README「用法」里写的命令，必须真的能解析，否则文档就是错的
        let args = Args::try_parse_from(
            ["tmdb-organizer", "./Bleach", "--order", "absolute", "--dry-run"],
        )
        .unwrap();
        assert_eq!(args.dir, std::path::PathBuf::from("./Bleach"));
        assert!(matches!(args.order, EpisodeOrder::Absolute));
        assert!(args.dry_run);
    }
}
```

⚠️ 三条：第一，README 里的命令改了参数名却没同步到 `Args`，`try_parse_from` 返回 `Err`、测试直接失败——**文档先于代码改了**；第二，`try_parse_from` 的第一个元素是**程序名**（惯例写 `"tmdb-organizer"`），漏掉它，真正的参数会被当成程序名，于是报缺参数 **the following required arguments were not provided**；第三，字段的 `///` 一改，`--help` 和 README 都要跟着改，**三处不一致的文档比没有文档更糟**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| README 只是说明文档 | 用法示例最好有测试兜底，否则会和代码脱节 |
| `--help` 和 README 可以各写各的 | 两者应同源：帮助文本就来自字段的 `///` |
| `try_parse_from` 直接传参数列表 | 第一个元素是程序名，必须留位 |
| README 写一次就不用再动 | 参数一改，示例、`--help`、README 三处要同步 |

**在 tmdb-organizer 里**：这一节对应仓库的 `README.md` 里"安装 / 用法 / 示例"三节；`usage_example_in_readme_parses` 这类测试放在 `src/main.rs` 或 D47 的 `tests/` 下，保证示例永远可运行。

### D63 · 复盘：参数组合的实际表现

📖 [docs](https://docs.rs/clap/latest/clap/_derive/index.html)

**参数是"组合"出来的，要一个个实测。** 单独测 `--order` 没问题，不代表 `--order absolute --dry-run -vv` 一起给也对。clap 把参数解析成结构体后，**各字段是独立的**，但它们在你的代码里怎么组合才决定真正的行为。所以复盘方式很直接：把想得到的组合都跑一遍，用一张"配置摘要"确认最终值。

**默认值 + 覆盖 = 最终配置。** 下面打印的四行，正是"没给就用默认、给了就覆盖"的结果：`./Bleach` 覆盖了 `dir`，`absolute` 覆盖了默认的 `tvdb`，`--dry-run` 把 `dry_run` 置真，`-vv` 把 `verbose` 累加为 2。**先打印配置、再执行动作，是 CLI 工具排错的通用套路。**

**该报错的组合要"报得准"。** 漏 `dir`、`--order` 给了非法值、多给了位置参数——这些都不是你的代码在处理，而是 clap 在解析阶段拦下并给出可读错误。**能交给 clap 的校验，就别在业务代码里重写一遍。**

```rust
use clap::{ArgAction, Parser, ValueEnum};
use std::path::PathBuf;

/// 整理《死神》剧集文件名
#[derive(Parser, Debug)]
#[command(name = "tmdb-organizer", version, about = "按 TMDB 集数顺序整理剧集文件")]
struct Args {
    /// 要扫描的目录
    dir: PathBuf,

    /// 集数顺序，默认 tvdb
    #[arg(long, value_enum, default_value_t = EpisodeOrder::Tvdb)]
    order: EpisodeOrder,

    /// 只打印计划，不真的改文件名
    #[arg(long)]
    dry_run: bool,

    /// 增加日志详细度，可重复：-v、-vv
    #[arg(short, long, action = ArgAction::Count)]
    verbose: u8,
}

#[derive(ValueEnum, Clone, Debug)]
enum EpisodeOrder {
    Tvdb,
    Dvd,
    Absolute,
}

fn main() {
    let args = Args::parse();
    // 一屏看清"默认值 + 用户覆盖"之后的最终配置
    println!("目录    : {}", args.dir.display()); // 目录    : ./Bleach
    println!("顺序    : {:?}", args.order); // 顺序    : Absolute
    println!("dry-run : {}", args.dry_run); // dry-run : true
    println!("verbose : {}", args.verbose); // verbose : 2
}
```

⚠️ 三条：第一，写成 `Args::parse().unwrap()`，报 **no method named unwrap found for struct Args**——`parse()` 出错时自己打印退出，返回值直接就是 `Args`，没有 `Result` 可 unwrap；第二，`--order` 给非法值或多给位置参数，clap 在解析阶段就报 **invalid value** / **unexpected argument**，并附上 Usage；第三，用 `try_parse_from` 做测试时漏掉第一个"程序名"元素，真正的参数会被当成程序名，报 **the following required arguments were not provided**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 参数各自测过就没问题 | 组合起来才是真实行为，值得逐个跑一遍 |
| `Args::parse()` 返回 `Result` | 直接返回 `Args`；解析失败会自己打印并退出 |
| 非法输入要自己写校验 | clap 在解析阶段就拦下，省掉大量手写判断 |
| 默认值是"代码里补的" | 默认值写进参数表（`default_value_t`），`--help` 里也会显示 |

**在 tmdb-organizer 里**：D63 把 D57-D61 的所有参数拼成最终 `Args`——`dir`、`--order`、`--dry-run`、`--verbose` 正是后面第 10-15 周一直沿用的 CLI 界面；**第 9 周到此收官——项目有了完整命令行入口，下一周开始接 TMDB API，把远程集数取回来。**

---

<a id="w10"></a>

## 第 10 周：HTTP 请求 TMDB，dotenv + ureq

| 天   | 学 6 min                                | 写 10 min                                                    | 验收 / commit          |
| ---- | --------------------------------------- | ------------------------------------------------------------ | ---------------------- |
| D64  | TMDB 申请 Key 与 `.env` 管理（[TMDB](https://developer.themoviedb.org/reference/search-tv)） | 创建 `.env` 写 `TMDB_API_KEY=...`，`.gitignore` 加 `.env`    | 不泄露 key，`day64`    |
| D65  | `dotenvy` 与 `ureq` 客户端（[docs](https://docs.rs/dotenvy/latest/dotenvy/)、[docs](https://docs.rs/ureq/latest/ureq/)） | `cargo add dotenvy ureq`，读取 API key                       | 打印 key 长度，`day65` |
| D66  | `ureq` GET 与查询参数（[docs](https://docs.rs/ureq/latest/ureq/)） | GET `/search/tv?query=Bleach&api_key=...`                    | 拿到响应，`day66`      |
| D67  | 读取响应体文本（[docs](https://docs.rs/ureq/latest/ureq/)） | 打印响应前 500 字符                                          | 看到 JSON，`day67`     |
| D68  | HTTP 状态码判断（[docs](https://docs.rs/ureq/latest/ureq/enum.Error.html)） | status 非 200 返回 Err                                       | 错误处理，`day68`      |
| D69  | `Box<dyn Error>` 封装（[std](https://rustwiki.org/zh-CN/std/error/trait.Error.html)） | `fn search_series(name: &str) -> Result<String, Box<dyn Error>>` | 编译通过，`day69`      |
| D70  | 复盘：把请求封装成可复用函数             | 搜索《死神》，打印 ID                                        | `cargo run`，`day70`   |

### D64 · TMDB 申请 Key 与 `.env` 管理

📖 [TMDB](https://developer.themoviedb.org/reference/search-tv)

**为什么要 key，以及为什么它绝不能写进代码。** TMDB 的接口靠 `api_key` 查询参数识别"谁在调用"，用来限流和追责。一旦 key 跟着 `git push` 进了公开仓库，它就是一次泄露的凭据——别人能拿你的配额甚至冒充你。所以铁律是：**代码里只出现变量名，真值放在 `.env`，而 `.env` 必须写进 `.gitignore`**。仓库里可以提交一份 `.env.example`（内容就是 `TMDB_API_KEY=你的key`）给人看格式，但真值永远只存在你本地。

**`.env` 生效靠"读进进程环境变量"，不是魔法。** 操作系统不会自动扫描你项目里的 `.env`。真正发生的事是：程序启动后显式读这个文件，把键值对塞进**当前进程**的环境变量表，之后用 `std::env::var` 才拿得到。这正是 D65 要 `dotenvy::dotenv()` 的原因。在这一天，先用纯 `std::env` 认识环境变量本身：它返回 `Result<String, VarError>`，读不到是 `NotPresent`，**不是空字符串**。

**`env::var` 返回 `Result`，正好复用第 4 周的错误处理。** 没读到 key 时别 `unwrap`，那会 panic；正确做法是 `match` 出 `NotPresent`，给一句"请创建 .env"的提示，把问题挡在发请求之前。还要意识到环境变量是**进程级**的：它不适合当敏感数据的日志来源——**打印长度可以，打印全文不行**。

```rust
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 真实项目里 key 来自 .env / 环境变量，绝不写死在代码里
    match env::var("TMDB_API_KEY") {
        Ok(key) => println!("已读到 key，长度 {} 个字符", key.len()),
        Err(env::VarError::NotPresent) => {
            // 没有 .env 时走这里：给出人能看懂的下一步
            eprintln!("没有找到 TMDB_API_KEY：请创建 .env 并写入 TMDB_API_KEY=你的key");
        }
        Err(e) => return Err(Box::new(e)),
    }
    Ok(())
}
```

⚠️ 三个坑：第一，key 没配就写 `env::var("TMDB_API_KEY").unwrap()`，运行时报 **called Result::unwrap() on an Err value: NotPresent** 并 panic，该用 `match` 或 `?`；第二，写成 `const KEY: &str = std::env::var(...)`，编译期直接报 **cannot call non-const function var in constants**——环境变量只能在运行时读；第三，`.env` 忘了加进 `.gitignore`，`git add .` 就把真 key 提交了，**这类错误没有任何编译错误提醒，只能靠纪律**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 把 key 写进代码、用完删掉就没事 | 只要提交过，Git 历史里就永远留着，必须去 TMDB 后台吊销重发 |
| `.env` 放在项目里程序就会自动读 | 不会，得由代码显式加载（D65 的 `dotenvy::dotenv()`） |
| 读不到环境变量会返回空字符串 | 返回 `Err(VarError::NotPresent)`，是错误而不是空串 |
| 可以 `println!` 出 key 方便调试 | 别这么做，日志和 CI 输出都可能被留存；最多打印长度 |

**在 tmdb-organizer 里**：这一步给第 10 周所有请求提供凭据来源——`.env` 里写 `TMDB_API_KEY=你的key`、`.gitignore` 加一行 `.env`、仓库里放 `.env.example` 说明格式；D65 起，`tmdb.rs` 里只出现变量名 `TMDB_API_KEY`，真值永不进代码。

### D65 · `dotenvy` 与 `ureq` 客户端

📖 [docs](https://docs.rs/dotenvy/latest/dotenvy/)、[docs](https://docs.rs/ureq/latest/ureq/)

**`dotenvy` 解决的是"谁来读 `.env`"。** `dotenvy::dotenv()` 会从当前目录往上找 `.env`，把内容读进**进程环境变量**，重复调用是幂等的；找不到文件时它返回 `Err`，用 `.ok()` 忽略即可——此时就依赖真正的外部环境变量（比如 CI 里注入的）。注意它**默认不覆盖**已存在的变量，所以"本地 `.env` 一定能覆盖系统变量"并不成立。

**`ureq` 是同步阻塞客户端，这是刻意的取舍。** 它**不需要 async 运行时**，`main` 里一行 `.call()` 就拿到结果，很适合"取一次数据就跑"的 CLI。代价是并发请求要自己开线程，但对 tmdb-organizer 完全够用。你还可以显式建一个 `Agent`：**连接池、超时、重定向、TLS 配置都挂在它身上**，多个请求复用一个 Agent 比每次用顶层 `ureq::get` 更省连接。

**两行"看起来没干啥但很关键"的代码。** 一是 `dotenvy::dotenv()`，它决定后面 `env::var` 能不能读到值；二是 `Agent::new_with_defaults()`，它**不联网**，只是搭好客户端。把"构造客户端"和"发请求"分开，测试时才方便把这层替换掉。

```rust
fn main() {
    // .env 不会自动生效：这一行把同目录下的 .env 读进进程环境变量
    // 找不到 .env 时它返回 Err，用 .ok() 忽略，交给下面的提示兜底
    dotenvy::dotenv().ok();

    let key = match std::env::var("TMDB_API_KEY") {
        Ok(k) => k,
        Err(_) => {
            eprintln!("缺少 TMDB_API_KEY：请创建 .env 写入 TMDB_API_KEY=你的key");
            eprintln!("并确认 .gitignore 里有一行 .env（否则会把 key 提交上去）");
            return;
        }
    };
    println!("key 已加载，长度 {} 个字符", key.len());

    // 复用同一个 Agent：连接池、超时、重定向策略都挂在它身上
    let agent = ureq::Agent::new_with_defaults();
    let _req = agent.get("https://api.themoviedb.org/3/search/tv"); // 只组装请求，尚未发送
    println!("Agent 已就绪，可以发请求了");
}
```

⚠️ 三条：第一，忘了 `cargo add dotenvy` 就直接用，报 **failed to resolve: use of unresolved module or unlinked crate dotenvy**，先补依赖；第二，漏掉 `dotenvy::dotenv()`，本地 `.env` 写了也白写，`env::var` 还是 `Err(NotPresent)`；第三，`dotenvy::dotenv().unwrap()` 在**没有 `.env`** 的环境（比如 CI）会 panic，应该用 `.ok()` 容忍。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 装好 `dotenvy` 就自动读 `.env` | 得自己调一次 `dotenvy::dotenv()`，依赖不会自己跑 |
| `.env` 一定会覆盖系统环境变量 | 默认不覆盖已存在的变量，取决于谁先设置 |
| `Agent::new_with_defaults()` 会先连一次服务器 | 只构造客户端，不联网；联网发生在 `.call()` |
| 同步客户端性能差、不该用 | 少了 async 运行时，代码和心智负担都小；这类 CLI 够用 |

**在 tmdb-organizer 里**：`tmdb.rs` 顶部先 `dotenvy::dotenv().ok()` 加载 `.env`，再从一个 `Agent` 发所有请求；`main` 里 key 缺失时只打印提示并正常退出，不去崩。

### D66 · `ureq` GET 与查询参数

📖 [docs](https://docs.rs/ureq/latest/ureq/)

**查询参数不要手拼字符串。** `.query("query", "Bleach")` 会做 URL 编码：空格变 `%20`、中文按 UTF-8 编码、`&`、`=`、`?` 这些分隔符也会被转义。手拼 `format!("?query={name}")` 只要 `name` 里有空格或中文就会拼出非法 URL——**编码交给库，别自己来**。多个 `.query()` 依次调用，最终拼成 `?query=Bleach&language=zh-CN&api_key=...`。

**`api_key` 走查询参数是 TMDB 的老式做法。** 更规范的是放到 `Authorization` 头（`.header("Authorization", &format!("Bearer {token}"))`），本教程沿用 TMDB 文档的 `api_key` 参数，简单直观。但要记住：**URL 会出现在日志、代理和浏览器历史里**，所以 key 放 query 天然不如放 header 安全。知道这个区别，比背 API 重要。

**`.call()` 是同步的、会阻塞。** 它返回 `Result<Response, ureq::Error>`：`Ok` 表示拿到了 HTTP 响应，`Err` 表示连请求都没成功（DNS、超时、TLS、连接被拒）。注意 ureq 3 里 **4xx/5xx 默认也算 `Err`**（`Error::StatusCode`），所以进 `Ok` 分支基本意味着 2xx——这点 D68 细讲。因为后面要读响应体，变量得声明成 `let mut resp`。

```rust
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let key = std::env::var("TMDB_API_KEY")?; // 没配置 key 就在这一行直接失败

    let mut resp = ureq::get("https://api.themoviedb.org/3/search/tv")
        .query("query", "Bleach") // 空格、中文等会被自动 URL 编码
        .query("language", "zh-CN")
        .query("api_key", &key)
        .call()?; // 真正发起网络请求（阻塞式）

    println!("状态码：{}", resp.status()); // 正常时是 200 OK
    let body = resp.body_mut().read_to_string()?;
    println!("响应体 {} 字节", body.len());
    Ok(())
}
```

⚠️ 三条：第一，后面要对 `resp` 调可变方法却写成 `let resp`，报 **cannot borrow resp as mutable, as it is not declared as mutable**，改成 `let mut resp`；第二，照抄 ureq 2.x 的 `.into_string()`，报 **no method named into_string found for enum Result**（3.x 改成了 `resp.body_mut().read_to_string()`）；第三，忘了 `.query("api_key", ...)`，服务端回 401，`call()` 直接是 `Err(Error::StatusCode(401))`——觉得"代码没问题"时，先怀疑请求里少了什么参数。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 查询参数要自己拼到 URL 后面 | 用 `.query()`，它负责 URL 编码；手拼遇到中文或空格就出错 |
| `api_key` 放哪都一样 | 放 URL 会进日志和历史，放 `Authorization` 头更安全 |
| `.call()` 的错误只是"网断了" | 还包括 4xx/5xx（默认转成 `Error::StatusCode`）、TLS、超时 |
| `resp` 直接就能读 body | 读 body 需要可变借用，变量得是 `let mut resp` |

**在 tmdb-organizer 里**：这几行是 `tmdb.rs` 里 `search_series(name)` 的雏形——把 `query`、`language`、`api_key` 三个参数拼好、发一次 GET，拿回 `Response` 交给 D67 读文本；第 11 周会把这段 JSON 交给 `SearchResponse` 解析。**真实运行需要你自己的 key，本教程的验证片段不联网。**

### D67 · 读取响应体文本

📖 [docs](https://docs.rs/ureq/latest/ureq/)

**响应体是"一次性的流"，读完就没了。** `read_to_string()` 把整个 body 读进内存并**消耗**它，再读第二次只会得到空串或报错。ureq 3 这个方法默认**限制 10MB**，并把非 UTF-8 字节做**有损替换**（`lossy`）——这是防止"一个超大响应把内存吃光"的保护，要读更大得显式 `.with_config().limit(...)`。

**先读文本、再解析，是排查问题的关键一步。** 直接反序列化失败时你根本不知道错在哪；先把原始 JSON 打印（或截断预览）出来，就能一眼看出是空数组、是错误页面、还是字段名对不上。**"解析失败先看原文"** 是接第三方 API 时最省时间的习惯。

**预览要按"字符"截，不能按"字节"截。** `&body[..500]` 是**字节**切片，一旦第 500 个字节落在某个中文或 emoji 的中间，就会 panic（见下）。正确做法是 `body.chars().take(500).collect::<String>()`，按 Unicode 标量值数个数，怎么截都不会切坏。

```rust
// 真实场景：let mut resp = ureq::get(url).call()?;
//           let body = resp.body_mut().read_to_string()?;
// 这里用内联字符串冒充"已经拿到的响应体"，离线也能复现打印效果
fn preview(body: &str, limit: usize) -> String {
    body.chars().take(limit).collect() // 只取前若干个字符，避免一屏刷到底
}

fn main() {
    let body = r#"{"page":1,"total_results":2,"results":[{"id":30984,"name":"BLEACH"},{"id":30985,"name":"BLEACH 千年血战篇"}]}"#;

    println!("响应前 500 字符：{}", preview(body, 500));
    println!("响应体总长度：{}", body.chars().count()); // 响应体总长度：104
}
```

⚠️ 三条：第一，`&body[..500]` 撞上中文边界，运行时报 **byte index 29 is not a char boundary; it is inside '千'**（字节数随内容变），换成 `.chars().take(n)`；第二，忘了 `?` 直接 `println!("{}", resp.body_mut().read_to_string())`，报 **Result<String, ureq::Error> doesn't implement std::fmt::Display**，要先 `?` 拿出来；第三，把 body 读两遍——第二次是空的，因为 body 已被消费。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 响应体可以反复读 | 读一次就消耗掉了，第二次是空的；需要留就自己存起来 |
| 截断随便按字节来 | 字节下标可能落在中文中间而 panic，要按 `.chars()` 数 |
| JSON 太长没法看 | 打印或截断预览，排查"解析失败"时最有用 |
| body 一定是 UTF-8 | 不一定，默认做有损替换，所以可能有个别字符变成替换符 |

**在 tmdb-organizer 里**：`search_series` 把响应体读成 `String` 后，先用 D100 的 `--verbose` 打印预览，再交给解析；如果解析失败，这段预览就是第一手证据。

### D68 · HTTP 状态码判断

📖 [docs](https://docs.rs/ureq/latest/ureq/enum.Error.html)

**ureq 默认就把 4xx/5xx 当错误，这比"自己判 200"更强的默认值。** 配置项 `http_status_as_error` 默认开启：服务端回 404、429、500 时，`.call()` 直接返回 `Err(Error::StatusCode(code))`。好处是"顺手 `?`"就能让错误浮上去；代价是**你必须 `match` 到 `Error::StatusCode` 才能拿到那个状态码**。而 3xx 重定向默认会被自动跟随。

**状态码要按"类别"判，而不是死等 200。** `2xx` 都算成功（`201 Created`、`204 No Content` 也是），标准库的 `is_success()` 正是判 `200..300`。死写 `status == 200` 会把 204 误判成失败。`StatusCode` 能和整数比较（`status == 200` 合法），但它**不是** `u16`，要取数字得 `.as_u16()`。

**"能拿到 status，说明已经成功"。** 因为 4xx/5xx 已经变成 `Err` 了。真要"无视状态码、自己判断"，可以 `.config().http_status_as_error(false).build()` 后再看 `resp.status()`——但多数情况不需要，**默认行为已经帮你做了想做的事**。

```rust
use std::error::Error;
use ureq::http::StatusCode;

// 和真实响应里的 resp.status() 是同一个类型
fn ensure_ok(code: u16) -> Result<StatusCode, Box<dyn Error>> {
    let status = StatusCode::from_u16(code)?; // 非法码（如 999）在这一步就失败
    if status.is_success() {
        Ok(status)
    } else {
        Err(format!("HTTP {status}：服务端拒绝了这次请求").into())
    }
}

fn main() {
    // ureq 默认会把 4xx/5xx 变成 Err(ureq::Error::StatusCode(code))，
    // 所以真实代码进 Ok 分支时已经是 2xx；这里手动演示判断过程
    match ensure_ok(200) {
        Ok(s) => println!("{s}：可以继续解析响应体"), // 200 OK：可以继续解析响应体
        Err(e) => println!("{e}"),
    }
    if let Err(e) = ensure_ok(404) {
        println!("{e}"); // HTTP 404 Not Found：服务端拒绝了这次请求
    }
}
```

⚠️ 三条：第一，`match ureq::get(url).call()` 只写了 `Ok` 分支，报 **non-exhaustive patterns: Err(_) not covered**——`Result` 必须两个都管；第二，把 `resp.status()` 直接赋给 `u16` 变量，报 **mismatched types: expected u16, found StatusCode**，要 `.as_u16()`；第三，以为"响应对了就一定是 200"，`201`、`204` 同样成功，用 `is_success()` 才对。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 只有 200 才算成功 | 2xx 都算成功，`is_success()` 判的是 `200..300` |
| 要自己写 `if resp.status() != 200` | 4xx/5xx 默认已变成 `Err`，能进 `Ok` 就说明成功 |
| `status` 就是个整数 | 是 `StatusCode`，能 `== 200` 比较，取数字要 `.as_u16()` |
| 404 会正常返回、由我判断 | 默认 `http_status_as_error` 开启，404 直接是 `Err` |

**在 tmdb-organizer 里**：`tmdb.rs` 的统一请求函数在这层把"服务端拒绝"翻译成人话（比如 `HTTP 404：没找到这个剧集`），让 `main` 只关心业务错误；D69 的 `Box<dyn Error>` 正是为了让这层能返回任何一种错误。

### D69 · `Box<dyn Error>` 封装

📖 [std](https://rustwiki.org/zh-CN/std/error/trait.Error.html)

**为什么需要"盒子错误"。** 一个函数里可能冒出好几种错误：`env::var` 的 `VarError`、`ureq` 的 `Error`、读 body 的 `io::Error`。若把返回类型写死某一种，其它两种就没法用 `?`。`Box<dyn Error>` 是**trait 对象**（D41）：把任意实现了 `Error` 的类型装进堆上的盒子，返回类型统一成一种，`?` 就能自动转换。

**它的代价和边界。** `Box<dyn Error>` 是**动态分发**：多一层指针间接、少一点优化，但对"启动时发几个请求"的 CLI 完全无感。两个实际限制要记住：默认的 `Box<dyn Error>` **不是 `Send`/`Sync`**，跨线程传递要写 `Box<dyn Error + Send + Sync>`；它**不携带具体类型**，调用方只能 `Display` 出信息，想按类型分支就得 `downcast`。第 15 周的 `anyhow` 就是在这层之上做了更顺手的封装。

**`?` 的自动转换是这里的核心。** `?` 会对错误调用 `From::from`：`VarError`、`ureq::Error`、`io::Error` 都实现了 `Into<Box<dyn Error>>`，所以同一函数里能混着用。标准库还专门提供了 `From<String>` 和 `From<&str>`，让 `format!(...)` 出来的消息也能直接当错误返回——这就是"给用户看的话"和"给程序看的类型"之间的桥。

```rust
use std::error::Error;

// 返回 Box<dyn Error>：底层是 io / http / utf-8 哪种错误都能装进来
fn search_series(name: &str) -> Result<String, Box<dyn Error>> {
    let key = std::env::var("TMDB_API_KEY")?; // VarError 自动转成 Box<dyn Error>
    let mut resp = ureq::get("https://api.themoviedb.org/3/search/tv")
        .query("query", name)
        .query("api_key", key)
        .call()?; // ureq::Error 同样自动转换
    let body = resp.body_mut().read_to_string()?;
    Ok(body)
}

fn main() {
    match search_series("Bleach") {
        Ok(body) => println!("拿到 {} 字节的响应", body.len()),
        // 未配置 key 时打印：搜索失败：environment variable not found
        Err(e) => eprintln!("搜索失败：{e}"),
    }
}
```

⚠️ 三条：第一，把返回类型写成 `Result<String, String>` 又用 `?` 接 `ureq::Error`，报 **the trait From<ureq::Error> is not implemented for String**；第二，在返回 `()` 的 `main` 里用 `?`，报 **the ? operator can only be used in a function that returns Result or Option**，要么改 `main -> Result<(), Box<dyn Error>>`，要么就地 `match`；第三，想把这个错误跨线程用，会报 **dyn std::error::Error cannot be sent between threads safely**，得加 `+ Send + Sync`。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `Box<dyn Error>` 是万能错误类型 | 不能按类型细分，跨线程还要 `+ Send + Sync`；大型项目更常用 `anyhow` |
| 一个函数只能返回一种错误 | `Box<dyn Error>` 下 `?` 会自动转换，多种错误能混用 |
| `Err("...")` 字符串不能当错误 | 标准库有 `From<&str>` 与 `From<String>`，可以直接 `?` 出来 |
| `Box<dyn Error>` 是静态分发、零开销 | 是 trait 对象，动态分发，多一次指针间接 |

**在 tmdb-organizer 里**：`search_series(name: &str) -> Result<String, Box<dyn Error>>` 就是第 10 周的最终签名——把"读 key、发请求、读 body"三步里的三类错误统一成一个返回类型，`main` 只需 `match` 一次。

### D70 · 复盘：把请求封装成可复用函数

📖 [docs](https://docs.rs/ureq/latest/ureq/)

**复盘的看点：一次请求里哪些是"变"的、哪些是"不变"的。** 不变的是：读 key、拼 URL、`.call()`、读 body、判断状态。变化的只有**搜索关键字**。把不变的部分收进一个函数、把变的部分做成参数，就是 `search_series(name)`——这正是 D6 讲的"函数抽公共部分"在真实场景里的落地。

**函数的边界要划在"返回文本"这一层。** 它**不解析 JSON**，只保证"拿到一段响应文本"。理由：解析属于第 11 周的职责，两者分开后，网络部分可以单独替换、单独测试（D55 那套思路）。返回值用 `Result<String, Box<dyn Error>>`：成功给文本，失败给盒子错误，调用方决定是打印还是降级。

**验收入口是"搜索《死神》、看到 id"。** 这里的 id 还没法靠 serde 拿（下一周才加），所以先用字符串截取"粗暴"地抠出来——**故意演示手写的脆弱**：`"id":` 一变形、出现嵌套就失效。看到它多容易坏，才知道下周引入 serde 值在哪。

```rust
use std::error::Error;

// 第 10 周的产出：一个"给名字、拿到响应体文本"的可复用函数
fn search_series(name: &str) -> Result<String, Box<dyn Error>> {
    let key = std::env::var("TMDB_API_KEY")?;
    let mut resp = ureq::get("https://api.themoviedb.org/3/search/tv")
        .query("query", name)
        .query("language", "zh-CN")
        .query("api_key", key)
        .call()?;
    Ok(resp.body_mut().read_to_string()?)
}

fn main() {
    // 离线演练：用内联 JSON 冒充 search_series 的返回值。
    // 真实跑法是 let body = search_series("Bleach")?;
    let body = r#"{"results":[{"id":30984,"name":"BLEACH"}]}"#;

    // 还没有 serde，先粗暴地抠出 id —— 这正是第 11 周要解决的事
    let id: String = body
        .split("\"id\":")
        .nth(1)
        .map(|s| s.chars().take_while(|c| c.is_ascii_digit()).collect())
        .unwrap_or_default();
    println!("《死神》的 TMDB id = {id}"); // 《死神》的 TMDB id = 30984

    if let Err(e) = search_series("Bleach") {
        println!("（真实请求需要先配置 key，当前失败：{e}）");
    }
}
```

⚠️ 三条：第一，`.nth(1)` 在 split 结果不足两个时返回 `None`，`unwrap_or_default()` 得到空串——**手写解析天然要把"没有"当正常情况**，这正是下周改用 serde 的动机；第二，`search_series` 里忘了 `let mut resp`，会报 **cannot borrow resp as mutable, as it is not declared as mutable**；第三，别把 `Err` 分支写成 `println!` 就完事——`main` 返回 `()` 时程序仍以退出码 0 结束，CI 里会"假装成功"；在 `main` 里硬用 `?` 则会报 **the ? operator can only be used in a function that returns Result or Option**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 封装就是把代码搬进函数 | 关键是划边界：函数只保证"拿到响应文本"，解析留给下一层 |
| 关键字里的空格要自己拼 `%20` | `.query()` 的值是整体，编码由库做，别手拼 |
| 手写 `split` 抠 id 也行，够用 | 嵌套、转义、字段顺序一变就失效，这正是要 serde 的原因 |
| `Err` 分支打印了就代表程序失败退出 | `main` 返回 `()` 仍是退出码 0；要显式 `exit(1)` 或返回 `Result` |

**在 tmdb-organizer 里**：`tmdb.rs` 的 `search_series(name)` 是第 10 周的收官产出；第 11 周会把它的返回文本喂给 `SearchResponse`，第 12 周再拿里面的 id 去查 Episode Groups。**第 10 周到此收官——项目从"只会读本地目录"升级成"能跟 TMDB 说上话"。**

---

<a id="w11"></a>

## 第 11 周：Serde JSON 反序列化

| 天   | 学 6 min                                  | 写 10 min                                                    | 验收 / commit      |
| ---- | ----------------------------------------- | ------------------------------------------------------------ | ------------------ |
| D71  | `serde` derive 与结构体建模（[docs](https://serde.rs/derive.html)） | `cargo add serde --features derive serde_json`，定义 `SearchResponse { results: Vec<SearchItem> }` | 编译通过，`day71`  |
| D72  | 字段属性与重命名（[docs](https://serde.rs/field-attrs.html)） | 定义 `SearchItem { id, name, first_air_date }`               | 编译通过，`day72`  |
| D73  | `from_str` 解析字符串（[docs](https://docs.rs/serde_json/latest/serde_json/fn.from_str.html)） | `serde_json::from_str::<SearchResponse>`                     | 解析成功，`day73`  |
| D74  | `Option<T>` 字段处理 null（[docs](https://serde.rs/field-attrs.html)） | `first_air_date: Option<String>`                             | 处理 null，`day74` |
| D75  | 数组取首元素与索引安全                    | 从 results 里取第一个 ID                                     | 打印 ID，`day75`   |
| D76  | 搜索逻辑封装与错误提示                    | `fn get_series_id(name: &str) -> Result<u32, Box<dyn Error>>` | 调用成功，`day76`  |
| D77  | 复盘：外部数据缺失时的降级                | 找不到剧集时返回 Err                                         | 错误提示，`day77`  |

### D71 · `serde` derive 与结构体建模

📖 [docs](https://serde.rs/derive.html)

**为什么不是"JSON 解析库"，而是"映射框架"。** `serde` 负责把数据在**任意格式**和 Rust 类型之间搬，`serde_json` 只是它的一种格式后端。核心是 `#[derive(Deserialize)]`：编译期为你的结构体生成"从 JSON 构造自己"的代码，规则是**按字段名逐字匹配**。所以"建模"比"解析"更重要——你写什么结构体，就等于声明了"我只关心这些字段"。

**多出来的字段会被忽略，缺字段才会报错。** TMDB 一条结果有二十来个字段，你的 `SearchItem` 只写 `id` 完全没问题：**未声明的键被静默丢弃**。反过来，声明了字段但 JSON 里没有，反序列化就会失败（除非加 `#[serde(default)]`，D72）。这条"宽进严出"的默认行为，决定了你该**按需建模**，而不是把整个响应抄一遍。

**类型即校验。** `id: u32` 不只是"装数字"，它同时声明了"JSON 里这一项必须是数字"。如果服务端哪天把 id 改成字符串，**解析会失败而不是悄悄变成 0**——这正是 D1 起反复讲的"用类型表达约束"，在外部数据边界上尤其值钱。

```rust
use serde::Deserialize;

// 顶层：TMDB 用 results 数组把搜索结果包起来
#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: Vec<SearchItem>,
}

// 每一条结果先只取 id，字段一多就需要 D72 的属性
#[derive(Debug, Deserialize)]
struct SearchItem {
    id: u32,
}

fn main() -> Result<(), serde_json::Error> {
    let body = r#"{"page":1,"results":[{"id":30984,"name":"BLEACH"}]}"#;
    let resp: SearchResponse = serde_json::from_str(body)?;
    // 结构体里没有的 page、name 会被直接忽略，不报错
    println!("{:?}", resp.results[0]); // SearchItem { id: 30984 }
    println!("id = {}", resp.results[0].id); // id = 30984
    Ok(())
}
```

⚠️ 三条：第一，漏写 `#[derive(Deserialize)]`，报 **the trait bound SearchResponse: serde::Deserialize is not satisfied**；第二，字段类型写宽了（比如把 `id` 写成 `String`），不报编译错，但解析真实数字时运行时报 **invalid type: integer 30984, expected a string**；第三，字段名拼错（比如把 `results` 写成 `result`），运行时报 **missing field results**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 要先把整个 JSON 的字段都建出来 | 只建你要用的字段，多出来的键会被忽略 |
| `#[derive(Deserialize)]` 是运行时反射 | 编译期生成的 `impl`，运行时无反射开销 |
| 字段类型随便写，反序列化会自己转 | 类型不匹配直接失败，正是它替你挡住了脏数据 |
| JSON 里的数组可以用结构体接 | 数组对应 `Vec<T>`，类型写错会报 **invalid type: sequence, expected a string** |

**在 tmdb-organizer 里**：`SearchResponse { results: Vec<SearchItem> }` 是第 11 周所有解析的入口类型，放在 `tmdb.rs`；第 12 周的 `EpisodeGroupResponse` 会沿用同一套"顶层 `results` 数组"的建模套路。

### D72 · 字段属性与重命名

📖 [docs](https://serde.rs/field-attrs.html)

**JSON 键名和 Rust 字段名常常对不上。** JSON 习惯 camelCase 与 snake_case 混用，Rust 字段则必须 snake_case。`#[serde(rename = "...")]` 就是这两套命名之间的**映射表**：告诉 serde"这个字段对应的是那个键"。想批量改风格用结构体级的 `#[serde(rename_all = "camelCase")]`，个别特例再用 `rename` 覆盖。

**`#[serde(default)]` 解决"键可能不存在"。** 注意区分两种"没有"：**键缺失**（JSON 里根本没这一项）和**值为 `null`**。`default` 管的是前者——缺失时用 `Default::default()`（`String` 是空串、数字是 0）。`null` 是另一回事，得靠 `Option<T>`（D74）。**把 `default` 当成 `Option` 的替代，是新手最常见的混淆。**

**为什么值得显式写出来。** 不写 `rename` 也能跑（只要键名恰好和字段名一样），但一旦你为了可读性把字段改名，就得靠它搭桥。**把"外部键名"和"内部字段名"解耦**，是让业务代码不受上游命名摆布的关键。

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: Vec<SearchItem>,
}

#[derive(Debug, Deserialize)]
struct SearchItem {
    id: u32,
    name: String,
    // JSON 键是 first_air_date，我们内部只想叫 aired，用 rename 搭桥
    #[serde(rename = "first_air_date")]
    aired: String,
    // 键名相同但不能缺：用 default 让缺失时落到 String 的默认值（空串）
    #[serde(default)]
    overview: String,
}

fn main() -> Result<(), serde_json::Error> {
    // 注意这里故意没给 overview，验证 default 是否生效
    let body = r#"{"results":[{"id":30984,"name":"BLEACH","first_air_date":"2004-10-05"}]}"#;
    let resp: SearchResponse = serde_json::from_str(body)?;
    let item = &resp.results[0];
    println!("{item:?}");
    // SearchItem { id: 30984, name: "BLEACH", aired: "2004-10-05", overview: "" }
    println!("id={} {} 首播于 {}，简介 {} 字", item.id, item.name, item.aired, item.overview.chars().count());
    // id=30984 BLEACH 首播于 2004-10-05，简介 0 字
    Ok(())
}
```

⚠️ 三条：第一，加了 `rename` 但 JSON 键仍缺失、又没 `default`，运行时报 **missing field first_air_date at line 1 column 28**；第二，以为重命名后旧键名还能用——不能，`rename` 之后**只认新键名**；第三，`default` 只补"键缺失"，碰到 `null` 照样失败，那时会报 **invalid type: null, expected a string**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `rename` 只是改个名字、旧名还能用 | 改了之后只认新键名，旧键被当成"未知字段"忽略 |
| `#[serde(default)]` 就是用来处理 null 的 | 它处理键缺失；null 要靠 `Option<T>` |
| `default` 会去调字段类型的构造函数 | 用的是 `Default::default()`，所以类型得实现 `Default` |
| 字段名必须和 JSON 键完全一致 | 不一致时用 `rename` 声明映射即可，字段名可以随你意 |

**在 tmdb-organizer 里**：`SearchItem` 的 `first_air_date`、`original_name` 等键名和你内部想用的名字不一致时，全在 `tmdb.rs` 的这一小组 `#[serde(...)]` 里解决，业务代码只看到自己定义的字段名。

### D73 · `from_str` 解析字符串

📖 [docs](https://docs.rs/serde_json/latest/serde_json/fn.from_str.html)

**`from_str` 的输入是 `&str`，输出是 `Result<T, serde_json::Error>`。** 它一次完成两件事：**语法**（这段文本是不是合法 JSON）和**结构映射**（能不能装进 `T`）。所以错误也分两类：语法错给 `expected value at line 1 column 1`，类型错给 `invalid type: ...`——**看错误里的行号列号，基本能定位到哪个字符**。

**为什么它不 panic。** 外部数据永远不可信，`from_str` 返回 `Result` 强迫你处理失败；这也让 D67 的"先看原文"顺理成章：解析失败时把原文一起打出来。**能用 `?` 传播就别 `unwrap`**。

**泛型参数怎么给。** 两种写法：`let x: SearchResponse = from_str(s)?;`（靠左侧类型标注推断）或 `from_str::<SearchResponse>(s)`（显式 turbofish）。**它们是同一件事**，编译器靠上下文推断 `T`；推断不出来时会报 `type annotations needed`。

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: Vec<SearchItem>,
}

#[derive(Debug, Deserialize)]
struct SearchItem {
    id: u32,
    name: String,
}

fn main() {
    // 1）正常：字符串 -> 结构体
    let good = r#"{"results":[{"id":30984,"name":"BLEACH"}]}"#;
    let resp: SearchResponse = serde_json::from_str(good).unwrap();
    let first = &resp.results[0];
    println!("解析成功：{} (id={})", first.name, first.id); // 解析成功：BLEACH (id=30984)

    // 2）类型不对：id 给成了字符串，from_str 返回 Err 而不是 panic
    let bad = r#"{"results":[{"id":"oops","name":"BLEACH"}]}"#;
    match serde_json::from_str::<SearchResponse>(bad) {
        Ok(_) => println!("不该走到这里"),
        // 解析失败：invalid type: string "oops", expected u32 at line 1 column 24
        Err(e) => println!("解析失败：{e}"),
    }
}
```

⚠️ 三条：第一，类型不对时报 **invalid type: string "oops", expected u32 at line 1 column 24**，列号指向出错的那个值，要么改模型要么改数据源；第二，忘了 `?`/`unwrap` 就直接 `println!("{}", serde_json::from_str(...))`，报 **Result<SearchResponse, serde_json::Error> doesn't implement std::fmt::Display**；第三，在返回 `Result<_, String>` 的函数里用 `?`，报 **the trait From<serde_json::Error> is not implemented for String**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `from_str` 失败会 panic | 返回 `Result`，不 panic；要不要 unwrap 由你决定 |
| 只要 JSON 合法就能解析成功 | 语法合法还不够，字段类型也得对得上，否则报 `invalid type` |
| `from_str` 会检查所有字段 | 它只校验你**声明了**的字段，其余忽略 |
| 解析错误看不出哪错 | 错误带行列号，能直接定位到出问题的那段文本 |

**在 tmdb-organizer 里**：`search_series` 拿到的 `String` 就是喂给 `serde_json::from_str::<SearchResponse>` 的原料；第 12 周解析 Episode Groups 还是同一个函数，只是 `T` 换成 `EpisodeGroupResponse`。

### D74 · `Option<T>` 字段处理 null

📖 [docs](https://serde.rs/field-attrs.html)

**`null` 和"缺字段"是两种"没有"，`Option<T>` 同时兜住它们。** serde 对 `Option<T>` 有特殊照顾：值为 `null` → `None`，键**根本不存在** → 也 `None`。所以 `first_air_date: Option<String>` 一次解决两种缺失，不用额外加属性。反过来，若写成 `first_air_date: String`，就会报 `invalid type: null, expected a string`。

**TMDB 的字段本来就常是 null。** 未定档的剧集没有首播日期，没海报的也没有 `poster_path`。**把这些字段建成 `String` 就等于假设"数据一定完整"**，一旦碰上这类条目整个解析就失败——注意是**整条响应失败**，不是只丢这一项。这正是 D13 讲 `Option` 的现实意义。

**`Option` 逼你在使用处做决定。** 拿到 `Option<String>` 后必须 `match`/`if let`，于是"没有日期时显示什么"被强制想清楚。**别急着 `.unwrap()`**：`null` 是**正常数据**，不是异常，unwrap 掉就是给自己埋雷。

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: Vec<SearchItem>,
}

#[derive(Debug, Deserialize)]
struct SearchItem {
    id: u32,
    name: String,
    // 未定档的剧集，TMDB 会给 null；也可能整个键都不出现
    first_air_date: Option<String>,
}

fn main() -> Result<(), serde_json::Error> {
    let body = r#"{"results":[
        {"id":1184,"name":"BLEACH","first_air_date":"2004-10-05"},
        {"id":99999,"name":"BLEACH 新作","first_air_date":null}
    ]}"#;
    let resp: SearchResponse = serde_json::from_str(body)?;
    for item in &resp.results {
        match &item.first_air_date {
            Some(d) => println!("[{}] {} 首播于 {d}", item.id, item.name),
            // [1184] BLEACH 首播于 2004-10-05
            None => println!("[{}] {} 尚未公布播出日期", item.id, item.name),
            // [99999] BLEACH 新作 尚未公布播出日期
        }
    }
    Ok(())
}
```

⚠️ 三条：第一，字段写成 `first_air_date: String` 而值是 null，运行时报 **invalid type: null, expected a string at line 1 column 56**；第二，对可能为 `None` 的字段直接 `.unwrap()`，运行时报 **called Option::unwrap() on a None value**，应改用 `match`/`unwrap_or_else`；第三，`Option<T>` 已自带"缺失即 None"，再叠 `#[serde(default)]` 是多余的——但**不会报错**，只是容易让人误以为两者机制相同。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `null` 和缺字段要分别处理 | `Option<T>` 把两者都变成 `None`，无需额外属性 |
| 一个字段是 null 只会让这一项失败 | 会导致**整个** `from_str` 失败，整批数据都拿不到 |
| `Option<T>` 字段该尽快 `unwrap` | null 是正常数据，应 `match` 出展示策略而不是 unwrap |
| `#[serde(default)]` 就是处理 null 的 | 它处理**键缺失**；null 要靠 `Option<T>` |

**在 tmdb-organizer 里**：`SearchItem.first_air_date` 用 `Option<String>`；到第 13 周生成重命名计划时，"没有首播日期"的条目要有明确的降级策略（跳过或标注），而不是让整个流程崩掉。

### D75 · 数组取首元素与索引安全

📖 [std](https://rustwiki.org/zh-CN/std/vec/struct.Vec.html)

**索引 `[0]` 会 panic，`first()` 不会。** `v[0]` 是"我相信至少有 1 个元素"，越界就 `index out of bounds`；`v.first()` 返回 `Option<&T>`，把"可能没有"编进类型，逼你处理空数组。**外部数据里"搜索不到"是完全正常的**（拼错剧名、TMDB 里确实没有），所以从一开始就该用 `first()`。

**`first()` 给的是引用，不是元素本身。** 返回 `Option<&SearchItem>`，因为数组还在 `resp` 里、不该被搬走（D9 的借用）。想在 `match` 里同时用它和别的字段，注意别和可变借用打架。

**`get(i)` 是 `first()` 的推广。** `v.get(0)` 与 `v.first()` 等价，`v.get(n)` 能取任意下标且同样安全；`&v[n]` 才是会 panic 的那个。**约定：下标来自自己写死的常量时可以用 `[i]`，来自外部数据时一律 `get`/`first`。**

```rust
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: Vec<SearchItem>,
}

#[derive(Debug, Deserialize)]
struct SearchItem {
    id: u32,
    name: String,
}

fn describe(body: &str) -> Result<String, Box<dyn Error>> {
    let resp: SearchResponse = serde_json::from_str(body)?;
    // resp.results[0] 在空数组时会 panic；first() 给出 Option，交给调用方决定
    match resp.results.first() {
        Some(item) => Ok(format!("{}（id={}）", item.name, item.id)),
        None => Err("搜索结果为空".into()),
    }
}

fn main() {
    let hit = r#"{"results":[{"id":30984,"name":"BLEACH"}]}"#;
    let miss = r#"{"results":[]}"#;
    println!("{:?}", describe(hit)); // Ok("BLEACH（id=30984）")
    println!("{:?}", describe(miss)); // Err("搜索结果为空")
}
```

⚠️ 三条：第一，空数组上写 `resp.results[0]`，运行时报 **index out of bounds: the len is 0 but the index is 0**；第二，把 `first()` 的返回值当元素直接点字段（`resp.results.first().id`），报 **no field id on type Option<&SearchItem>**，得先 `match`/`if let`；第三，`.first().unwrap()` 只是把 panic 往后挪一步，空数组时照样报 **called Option::unwrap() on a None value**——要安全就用 `match` 或 `ok_or_else` 转成错误。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `results[0]` 取不到就是 `None` | 直接 panic `index out of bounds`；要 `Option` 得用 `first()` |
| `first()` 返回元素本身 | 返回 `Option<&T>`（借用），不是一个独立的值 |
| `.first().unwrap()` 就安全了 | 空数组时它照样 panic，跟 `[0]` 没本质区别 |
| 搜索不到是异常 | 是正常业务结果，应转成 `Err`/`None` 而不是崩 |

**在 tmdb-organizer 里**：`get_series_id` 里"从 `results` 取第一条"就靠 `first()`；"搜不到"会走 `PlanStatus::Missing` 那类分支（第 13 周），而不是 panic——**外部数据永远不能假设非空。**

### D76 · 搜索逻辑封装与错误提示

📖 [std](https://rustwiki.org/zh-CN/std/option/enum.Option.html)

**封装的关键是"返回类型说清全部可能"。** `Result<u32, Box<dyn Error>>` 一句话交代：给我剧名，要么给我 id，要么给我一个能 `Display` 的错误。**缺 key、请求失败、JSON 不合法、搜不到**——这四种对内是不同的错误，对外都收敛成一个 `Result`。这正是 D69 的价值。

**错误提示要"对用户有用"。** "没有找到剧集：xxx"比"解析失败"强，因为它告诉用户**下一步该做什么**（换个关键词）。把 `format!` 出来的消息用 `.ok_or_else` 挂到 `None` 上，字符串会自动转成 `Box<dyn Error>`。**`ok_or_else` 与 `ok_or` 的区别**：前者接收闭包、**惰性**构造错误，后者总会先构造出 `Err` 值——带 `format!` 时永远选 `ok_or_else`。

**函数越小越好测。** `get_series_id` 只做"解析 + 取第一条 + 给错"，网络交给外面的 `search_series`。这样测试时把 JSON 字符串直接喂进来，**不用联网**（本教程的离线演练就是这么做的）。

```rust
use serde::Deserialize;
use std::error::Error;

#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: Vec<SearchItem>,
}

#[derive(Debug, Deserialize)]
struct SearchItem {
    id: u32,
    name: String,
}

// 真实场景：body 来自第 10 周的 search_series(name)
fn get_series_id(name: &str) -> Result<u32, Box<dyn Error>> {
    // 离线演练：内联 JSON 代替网络响应，让函数可以脱网测试
    let body = if name.eq_ignore_ascii_case("bleach") {
        r#"{"results":[{"id":30984,"name":"BLEACH"}]}"#
    } else {
        r#"{"results":[]}"#
    };

    let resp: SearchResponse = serde_json::from_str(body)?;
    let item = resp
        .results
        .first()
        .ok_or_else(|| format!("没有找到剧集：{name}"))?; // None -> 带名字的错误
    println!("匹配到：{}", item.name); // 匹配到：BLEACH
    Ok(item.id)
}

fn main() {
    match get_series_id("Bleach") {
        Ok(id) => println!("《死神》id = {id}"), // 《死神》id = 30984
        Err(e) => eprintln!("失败：{e}"),
    }
    if let Err(e) = get_series_id("不存在的剧") {
        println!("失败：{e}"); // 失败：没有找到剧集：不存在的剧
    }
}
```

⚠️ 三条：第一，`ok_or("没有找到：{}")` 忘了用 `format!`（少了参数），报 **1 positional argument in format string, but no arguments were given**；第二，在返回 `()` 的 `main` 里对 `Result` 用 `?`，报 **the ? operator can only be used in a function that returns Result or Option**；第三，忘了 `first()` 直接 `resp.results[0]`，"搜不到"时会 panic **index out of bounds: the len is 0 but the index is 0**——这恰恰是最该被转成 `Err` 的那种情况。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 错误信息随手写就行 | 要给用户看、能指导下一步；内部错误细节留给日志 |
| `ok_or` 和 `ok_or_else` 只是写法不同 | 后者惰性；配 `format!` 时不会白白构造错误 |
| 这类函数必须联网才能测 | 只吃字符串输入，用内联 JSON 就能离线测 |
| `Box<dyn Error>` 太模糊，不如自定义枚举 | 起步阶段够用；第 15 周的 `anyhow` 会在它之上做更顺手的封装 |

**在 tmdb-organizer 里**：`get_series_id(name) -> Result<u32, Box<dyn Error>>` 是第 11 周的收官函数，第 12 周拿这个 id 去请求 `/tv/{id}/episode_groups`。

### D77 · 复盘：外部数据缺失时的降级

📖 [docs](https://docs.rs/serde_json/latest/serde_json/)

**降级的核心问题：缺哪一块，还能不能继续往下走。** 三种缺失要分开：**解析层**（响应根本不是 JSON，比如网关返回 HTML）——直接 `Err`，没法继续；**业务层**（JSON 合法但 `results` 为空）——也是 `Err`，但提示要给"换个关键词"这种人话；**字段层**（单条数据里某字段是 `null`）——**不该让它拖垮整条结果**，用 `Option` 兜住（D74）。

**分层决定"谁兜底"。** 解析错误和空结果都是**终结性**的，交给返回 `Err`、把决定权交给调用方；字段缺失是**局部性**的，就在模型层用 `Option` 消化掉。**别把两类混在一起**：如果为了"容错"把所有字段都写成 `Option`，你会失去"这个字段本不该缺"的类型保证。

**错误信息是产品的一部分。** 同一次失败，`响应不是预期格式` 告诉用户"可能是网络或网关问题"，`TMDB 没返回任何结果` 告诉用户"换个词"。**把 `map_err` 用起来**，就能在最外层给出分层的、可行动的提示，而不是把 `serde_json` 的原始错误直接甩出去。

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct SearchResponse {
    results: Vec<SearchItem>,
}

#[derive(Debug, Deserialize)]
struct SearchItem {
    id: u32,
    name: String,
}

// 降级策略：能解析就交出第一个结果，解析不了就给一句人能看懂的提示
fn find_series(body: &str) -> Result<SearchItem, String> {
    let resp: SearchResponse =
        serde_json::from_str(body).map_err(|e| format!("响应不是预期格式：{e}"))?;
    resp.results
        .into_iter()
        .next()
        .ok_or_else(|| "TMDB 没返回任何结果：换个关键词或加上年份再搜".to_string())
}

fn main() {
    let ok = r#"{"results":[{"id":30984,"name":"BLEACH"}]}"#;
    let empty = r#"{"results":[]}"#;
    let broken = r#"<html>504 Gateway Timeout</html>"#;

    match find_series(ok) {
        Ok(item) => println!("找到《{}》，id={}", item.name, item.id), // 找到《BLEACH》，id=30984
        Err(e) => println!("{e}"),
    }
    println!("{}", find_series(empty).unwrap_err());
    // TMDB 没返回任何结果：换个关键词或加上年份再搜
    println!("{}", find_series(broken).unwrap_err());
    // 响应不是预期格式：expected value at line 1 column 1
}
```

⚠️ 三条：第一，函数返回 `Result<_, String>` 却对 `serde_json::Error` 直接 `?`，报 **the trait From<serde_json::Error> is not implemented for String**——要么 `map_err`，要么改用 `Box<dyn Error>`；第二，对确定成功的 `Result` 调 `.unwrap_err()`，运行时报 **called Result::unwrap_err() on an Ok value: 30984**；第三，给 `Option` 用 `.map_err(...)`，报 **no method named map_err found for enum Option<T>**——`map_err` 只属于 `Result`，`Option` 要用 `ok_or_else`。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 所有容错都靠 `Option` 就够了 | 解析失败/空结果该 `Err`；字段缺失才用 `Option` |
| 出错直接把原始错误抛给用户 | 用 `map_err` 分场景翻译成可行动的人话 |
| 网关返回 HTML 时也当"格式错误"处理 | 那是请求层失败，最好在状态码或内容类型就拦下（D68） |
| 降级就是不报错、静默跳过 | 降级也要有明确信号；静默跳过是第 13 周 `PlanStatus` 要解决的问题 |

**在 tmdb-organizer 里**：第 11 周到此收官——`tmdb.rs` 能把响应解析成 `SearchResponse`、取出 `SearchItem`，并在"搜不到或格式不对"时给出分层提示；第 12 周开始用同样的套路解析 Episode Groups，把不同 Order 的集数拿回来。**第 11 周到此收官——项目从"能拿到一段 JSON 文本"升级成"能把文本变成有类型的剧集数据"。**

---

<a id="w12"></a>

## 第 12 周：TMDB Episode Groups API，拿到不同 Order

| 天   | 学 6 min                                | 写 10 min                                                    | 验收 / commit               |
| ---- | --------------------------------------- | ------------------------------------------------------------ | --------------------------- |
| D78  | Episode Groups 接口与返回结构（[TMDB](https://developer.themoviedb.org/reference/search-tv)） | 请求 `/tv/{series_id}/episode_groups`，打印 JSON             | 看到 groups，`day78`        |
| D79  | `results` 数组嵌套反序列化（[docs](https://serde.rs/derive.html)） | 定义 `EpisodeGroupResponse { results: Vec<EpisodeGroup> }`   | 解析成功，`day79`           |
| D80  | 按字段筛选分组                          | 定义 `EpisodeGroup { id, name, group_count }`，列出所有组    | 找到 TVDB/Absolute，`day80` |
| D81  | 详情接口与 URL 拼接                     | 请求 `/tv/{id}/episode_group/{group_id}`                     | 拿到详情，`day81`           |
| D82  | 三层嵌套结构体建模                      | 定义 `GroupDetail { groups: Vec<Group> }`、`Group { episodes }`、`GroupEpisode { season_number, episode_number, name }` | 解析成功，`day82`           |
| D83  | 远程数据结构转换成本地模型              | 把 `GroupEpisode` 转成你的 `Episode`                         | 生成 Vec，`day83`           |
| D84  | 复盘：不同 Order 的数据差异             | 打印 TVDB Order 前 5 集                                      | `cargo run`，`day84`        |

### D78 · Episode Groups 接口与返回结构

📖 [TMDB](https://developer.themoviedb.org/reference/search-tv)

**为什么同一部剧会有好几套"集数编号"。** TMDB 的 `/tv/{series_id}` 只给**默认（首播）顺序**，但《死神》这类长篇动画的编法不止一种：按电视播出季切开（TVDB Order）、按 DVD 发售切开（DVD Order）、以及从第 1 集一路数到底（Absolute Order）。三者对"第几集"的答案不同，所以 TMDB 把每种排序单独存成一个 **Episode Group**（分组），用 `/tv/{series_id}/episode_groups` 列出全部可用分组。

**这个接口只给"目录"，不给"剧集本身"。** 列表里每个分组只有 `id`、`name`、`group_count`、`episode_count`、`type`——够你挑选用哪一套，但拿不到每集的名字。真正的集数还得拿分组 `id` 去请求 D81 的详情接口。这种"先列目录、再取详情"的两段式，是 REST API 里很常见的形状。

**为什么先摊开 JSON 再建模。** 官方文档和实际字段偶尔有出入，而你只需要几个键。**先用 `serde_json::Value` 看一眼原文**，确认键名和类型（分组 `id` 是**十六进制字符串**、`network` 可能是 `null`），再回头写 struct（D79）——这正是 D67"先看原文"的延续。

```rust
fn main() {
    // 真实场景：ureq 请求 https://api.themoviedb.org/3/tv/30984/episode_groups 拿到的响应体
    let body = r#"{
        "id": 30984,
        "results": [
            {"id": "5cff5b6bc3a3687dd10001db", "name": "Bleach TVDB Order", "type": 6, "group_count": 16, "episode_count": 366},
            {"id": "5cff5b6ec3a36826290001e1", "name": "Bleach Absolute Order", "type": 6, "group_count": 1, "episode_count": 366}
        ]
    }"#;

    // 先不建模，用 Value 把原文摊开：确认键名与类型
    let v: serde_json::Value = serde_json::from_str(body).unwrap();
    println!("剧集 id = {}", v["id"]); // 剧集 id = 30984
    for g in v["results"].as_array().unwrap() {
        println!(
            "{} / {} ({} 集)",
            g["name"].as_str().unwrap(),
            g["id"].as_str().unwrap(),
            g["episode_count"].as_u64().unwrap()
        );
        // Bleach TVDB Order / 5cff5b6bc3a3687dd10001db (366 集)
        // Bleach Absolute Order / 5cff5b6ec3a36826290001e1 (366 集)
    }
}
```

⚠️ 三条：第一，分组的 id 在 JSON 里是**带引号的字符串**，用 `as_u64()` 取只会得到 `None`，`unwrap()` 直接 panic **called Option::unwrap() on a None value**；第二，键名写错（`results` 写成 `result`）**不会报错**，`v["result"]` 只是默默给出 `Null`，接着 `as_array().unwrap()` 同样 panic **called Option::unwrap() on a None value**——取 `Value` 的每一步都得留退路；第三，`Value` 不校验类型，`as_u64` / `as_str` 取错类型也只在运行时才知道，字段一旦确定就该照 D79 换成 struct，让编译器替你检查。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 列表接口能直接拿到每集标题 | 只给分组目录，集数要再请求详情接口（D81） |
| 分组的 `id` 是数字 | 是十六进制字符串，必须用 `String` 接 |
| 用 `Value` 取不存在的键会报错 | 得到 `Value::Null`，后面的 `.as_array().unwrap()` 才 panic |
| 一部剧只有一种顺序 | 常有 TVDB / DVD / Absolute 多个分组并存 |

**在 tmdb-organizer 里**：`tmdb.rs` 里新增 `list_episode_groups(series_id)`，把响应体原样交给解析层；第 12 周先把"目录"这一步调通，再决定用哪套顺序去匹配本地文件（第 13 周）。

### D79 · `results` 数组嵌套反序列化

📖 [docs](https://serde.rs/derive.html)

**嵌套数组不是特殊语法，只是"字段类型本身又是一个结构体"。** 顶层写成 `EpisodeGroupResponse { results: Vec<EpisodeGroup> }`，serde 看到 `Vec<T>` 就逐个元素去构造 `T`。**数组有多深，就对应几层 `Vec`/struct，一层都不能少**：把 `results` 写成本身不是 `Vec` 的类型，解析直接失败。

**同一个响应里两种 id，说明"建模必须照着数据来"。** 顶层 `id: 30984` 是数字，分组 `id` 却是 `5cff5b6bc3a3687dd10001db` 这样的**十六进制字符串**，写成 `u32` 就会解析失败。分组里还有个 `type` 字段，它是 Rust 关键字，只能改名后靠 `#[serde(rename)]` 搭桥（D72）。

**建模顺序：从叶子往上写。** 先写最小的 `EpisodeGroup`，再写包住它的 `EpisodeGroupResponse`。**只声明你要用的字段**（D71）：`description`、`network` 先不管，多出来的键会被忽略。

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EpisodeGroupResponse {
    results: Vec<EpisodeGroup>, // JSON 数组 -> Vec<T>，逐个元素构造 T
}

#[derive(Debug, Deserialize)]
struct EpisodeGroup {
    id: String, // 十六进制字符串，不能用 u32
    name: String,
}

fn main() -> Result<(), serde_json::Error> {
    let body = r#"{"id":30984,"results":[
        {"id":"5cff5b6bc3a3687dd10001db","name":"Bleach TVDB Order","episode_count":366},
        {"id":"5cff5b6ec3a36826290001e1","name":"Bleach Absolute Order","episode_count":366}
    ]}"#;

    let resp: EpisodeGroupResponse = serde_json::from_str(body)?;
    println!("共 {} 个分组", resp.results.len()); // 共 2 个分组
    for (i, g) in resp.results.iter().enumerate() {
        println!("[{}] {} ({})", i, g.name, g.id);
        // [0] Bleach TVDB Order (5cff5b6bc3a3687dd10001db)
        // [1] Bleach Absolute Order (5cff5b6ec3a36826290001e1)
    }
    Ok(())
}
```

⚠️ 三条：第一，把 `id` 写成 `u32`，运行时报 **invalid type: string "5cff5b6bc3a3687dd10001db", expected u32**，错误会指出出错的位置；第二，把 `results` 写成 `String` 这类非数组类型，报 **invalid type: sequence, expected a string**——数组必须由 `Vec<T>` 接；第三，键名对不上（写成 `result`）报 **missing field result**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 嵌套数组要手写多层解析 | 类型里写几层 `Vec`/struct 就够，serde 自动递归 |
| 响应里的 id 都是数字 | 顶层 id 是数字，分组 id 是字符串 |
| 顶层只声明 `results` 会漏数据 | 用不到的键本来就被忽略，这正是"按需建模" |
| 字段名必须和 JSON 键完全一致 | 关键字如 `type` 要靠 `#[serde(rename)]` 搭桥 |

**在 tmdb-organizer 里**：`EpisodeGroupResponse` 与 `SearchResponse` 是同一套"顶层 `results` 数组"的套路，都放 `tmdb.rs`；`EpisodeGroup` 是它的元素类型，D80 会再补上 `group_count`、`type` 两个字段。

### D80 · 按字段筛选分组

📖 [docs](https://serde.rs/field-attrs.html)

**筛选的分界线：远程数据是"多选"，本地需求是"单选"。** 一个剧可能同时有 TVDB / DVD / Absolute 好几个分组，但第 13 周的 `build_plan` 一次只用一套编号。所以先**列出全部**给人看，再**按关键字或类型挑一个**——和第 9 周 clap 的 `ValueEnum` 是同一个"先枚举、再挑选"的思路。

**别用精确相等去匹配名字。** TMDB 的分组名带剧名前缀（`Bleach TVDB Order`），写 `g.name == "TVDB"` 永远匹配不到；**按关键字 `contains` 匹配更耐用**，比较前统一 `to_ascii_lowercase` 还能避开大小写坑。要更严谨就按 `type` 字段分——它是 TMDB 用来区分分组用途的数字编码，官方文档里各有含义。

**返回引用而不是克隆。** `find_group` 返回 `Option<&EpisodeGroup>`：分组还在原 `Vec` 里，挑选只是"借一个出来"，**没必要 `.clone()`**。坚持这个习惯，到第 13 周 `build_plan` 才能全程借用、零拷贝。

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct EpisodeGroupResponse {
    results: Vec<EpisodeGroup>,
}

#[derive(Debug, Deserialize)]
struct EpisodeGroup {
    id: String,
    name: String,
    group_count: u32,
    #[serde(rename = "type")] // type 是 Rust 关键字，字段只能改名后靠 rename 搭桥
    group_type: u32,
}

// 按名字里的关键字找分组；找不到返回 None，交给调用方决定
fn find_group<'a>(groups: &'a [EpisodeGroup], keyword: &str) -> Option<&'a EpisodeGroup> {
    let kw = keyword.to_ascii_lowercase();
    groups
        .iter()
        .find(|g| g.name.to_ascii_lowercase().contains(&kw))
}

fn main() -> Result<(), serde_json::Error> {
    let body = r#"{"id":30984,"results":[
        {"id":"5cff5b6bc3a3687dd10001db","name":"Bleach TVDB Order","type":6,"group_count":16,"episode_count":366},
        {"id":"5cff5b6ec3a36826290001e1","name":"Bleach Absolute Order","type":6,"group_count":1,"episode_count":366}
    ]}"#;
    let resp: EpisodeGroupResponse = serde_json::from_str(body)?;

    for g in &resp.results {
        println!("{} (type={}, {} 个子分组)", g.name, g.group_type, g.group_count);
        // Bleach TVDB Order (type=6, 16 个子分组)
        // Bleach Absolute Order (type=6, 1 个子分组)
    }
    match find_group(&resp.results, "tvdb") {
        Some(g) => println!("选中 TVDB 分组：{}", g.id), // 选中 TVDB 分组：5cff5b6bc3a3687dd10001db
        None => println!("没有 TVDB 顺序，回退到默认顺序"),
    }
    Ok(())
}
```

⚠️ 三条：第一，字段直接命名为 `type`，编译报 **expected identifier, found keyword type**——`type` 是关键字，必须改名再加 `#[serde(rename = "type")]`；第二，`find_group` 返回引用却忘了标生命周期，报 **missing lifetime specifier**，提示会问"这个借用来自 groups 还是 keyword"；第三，大小写不统一（用 `"TVDB"` 去比 `bleach tvdb order`）匹配不到但**不报错**，只会静默走到 `None` 分支，所以比较前先 `to_ascii_lowercase`。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 分组名就是 `"TVDB"` | 实际带剧名前缀，得用 `contains` 关键字匹配 |
| 找不到分组应该 panic | 返回 `None`，让调用方回退到默认顺序 |
| `find` 返回元素本身 | 返回 `Option<&T>`，只是借用一个 |
| `type` 可以直接当字段名 | 关键字不能作标识符，得 `rename` 搭桥 |

**在 tmdb-organizer 里**：`find_group(&groups, "tvdb")` 决定第 13 周用哪套编号；`group_type` 先打印出来供人工核对，以后若要按类型自动挑选也用它。

### D81 · 详情接口与 URL 拼接

📖 [std](https://rustwiki.org/zh-CN/std/macro.format.html)

**URL 拼接的难点不是拼，而是"类型对不上就拼不进去"。** Rust 的 `+` 只允许 `String + &str`：左边必须是有所有权的 `String`，右边是借用，`&str + &str` 直接编译失败。`format!` 没这个限制——**任何实现 `Display` 的类型都能进占位符**，`u32` 的 `series_id` 不用手动 `.to_string()`。所以拼 URL 这件事，`format!` 几乎是唯一省心的选择。

**两段式接口的第二个 URL 才带 group_id。** 详情接口是 `/tv/{series_id}/episode_group/{group_id}`，注意路径里是**单数** `episode_group`，而 D78 的列表接口是复数 `episode_groups`——一字之差就是 404。十六进制 group_id 只含 `0-9a-f`，属于 URL 安全字符，**不需要百分号编码**。

**key 不放进 URL 里。** 第 10 周讲过，`api_key` 既能拼查询串也能放请求头；**拼进 URL 容易被日志和终端历史记下来**，所以统一走请求头。这里只负责拼"路径部分"。

```rust
// 详情接口路径：/tv/{series_id}/episode_group/{group_id}
fn group_detail_url(series_id: u32, group_id: &str) -> String {
    format!("https://api.themoviedb.org/3/tv/{series_id}/episode_group/{group_id}")
}

fn main() {
    // group_id 来自 D80 筛出来的那个分组，是十六进制字符串，直接用
    let series_id = 30984;
    let group_id = "5cff5b6bc3a3687dd10001db";

    let url = group_detail_url(series_id, group_id);
    println!("{url}");
    // https://api.themoviedb.org/3/tv/30984/episode_group/5cff5b6bc3a3687dd10001db
    println!("长度 {} 字节", url.len()); // 长度 76 字节
}
```

⚠️ 三条：第一，用 `+` 拼两个 `&str`（`"https://..." + group_id`），编译报 **cannot add &str to &str**；第二，想用 `String + series_id` 省掉 `format!`，报 **mismatched types: expected &str, found u32**；第三，把 `episode_group` 写成 `episode_groups`（或反之）**不报错**，只在请求时得到 404——所以先打印 URL 再发请求，用浏览器或 `curl` 手工核对一次。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 拼接字符串用 `+` 最方便 | 只支持 `String + &str`，类型混着拼就得用 `format!` |
| `format!` 里的数字要先 `to_string()` | 占位符直接吃 `Display`，数字原样写 |
| 列表和详情的路径一样 | 列表是复数 `episode_groups`，详情是单数 `episode_group` |
| group_id 里有特殊字符要转义 | 十六进制字符串是 URL 安全字符，直接拼 |

**在 tmdb-organizer 里**：`group_detail_url(series_id, group_id)` 放在 `tmdb.rs`，和 `search_series`、`list_episode_groups` 并排；返回值直接喂给第 10 周的 `ureq` GET，响应体再交给 D82 的解析函数。

### D82 · 三层嵌套结构体建模

📖 [docs](https://serde.rs/derive.html)

**层级不是"设计"，是照着 JSON 抄。** 响应是 `根 { groups: [ { episodes: [ {...} ] } ] }`，那就三层 struct 加两个 `Vec`：`GroupDetail` → `Vec<Group>` → `Vec<GroupEpisode>`。**每一层只描述那一层**，上游任意一层加字段都不影响其它层。

**同名 `name` 出现三次，靠位置区分语义。** `GroupDetail.name` 是分组名（`Bleach TVDB Order`）、`Group.name` 是段名（`Season 1`）、`GroupEpisode.name` 是集标题。**Rust 不会因为同名而冲突，它们是不同类型上的字段**；但读代码的人容易混，变量名要起清楚（`ep.name` 比裸 `name` 明确）。

**为什么不直接复用本地的 `Episode`。** 远程字段叫 `season_number` / `episode_number`，本地模型叫 `season` / `episode`（D83 才转换）。**把"上游形状"和"本地形状"分成两套类型**，上游改字段名时只动一个转换函数，业务代码毫发无伤——这就是防腐层。

```rust
use serde::Deserialize;

// 第一层：详情响应
#[derive(Debug, Deserialize)]
struct GroupDetail {
    name: String,
    groups: Vec<Group>,
}

// 第二层：子分组（一段 / 一季）
#[derive(Debug, Deserialize)]
struct Group {
    name: String,
    order: u32,
    episodes: Vec<GroupEpisode>,
}

// 第三层：单集
#[derive(Debug, Deserialize)]
struct GroupEpisode {
    season_number: u32,
    episode_number: u32,
    name: String,
}

fn main() -> Result<(), serde_json::Error> {
    let body = r#"{
        "id": "5cff5b6bc3a3687dd10001db",
        "name": "Bleach TVDB Order",
        "groups": [
            {"id": "abc", "name": "Season 1", "order": 0, "episodes": [
                {"season_number": 1, "episode_number": 1, "name": "The Day I Became a Shinigami", "order": 1},
                {"season_number": 1, "episode_number": 2, "name": "A Shinigami's Work", "order": 2}
            ]},
            {"id": "def", "name": "Season 2", "order": 1, "episodes": [
                {"season_number": 2, "episode_number": 1, "name": "The Nightmare Returns", "order": 1}
            ]}
        ]
    }"#;

    let detail: GroupDetail = serde_json::from_str(body)?;
    println!("{}：{} 个子分组", detail.name, detail.groups.len()); // Bleach TVDB Order：2 个子分组
    for g in &detail.groups {
        println!("{}（order={}，{} 集）", g.name, g.order, g.episodes.len());
        for ep in &g.episodes {
            println!("  S{:02}E{:02}  {}", ep.season_number, ep.episode_number, ep.name);
        }
    }
    // Season 1（order=0，2 集）
    //   S01E01  The Day I Became a Shinigami
    //   S01E02  A Shinigami's Work
    // Season 2（order=1，1 集）
    //   S02E01  The Nightmare Returns
    Ok(())
}
```

⚠️ 三条：第一，漏掉中间一层（把 `groups` 写成 `Vec<GroupEpisode>`），运行时报 **missing field season_number**——serde 拿着"分组对象"去找单集的字段，自然找不到；第二，字段名拼错（`episodes` 写成 `episode`），报 **missing field episodes**；第三，把分组外层的 `order`（段序，从 0 开始）当成集号用，编号会整体偏移，**两个都是 `u32`，编译器不会拦你**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 嵌套 JSON 要用 `Value` 层层剥 | 声明三层 struct，serde 自动递归构造 |
| 同名的 `name` 会冲突 | 分属不同类型，字段同名互不影响 |
| 中间层可以省掉直接取 `episodes` | 层级对应 JSON 结构，省一层就 `missing field` |
| 远程结构可以直接当业务模型 | 两套类型加一个转换函数，才能隔离上游变化 |

**在 tmdb-organizer 里**：`GroupDetail` / `Group` / `GroupEpisode` 三个类型都放 `tmdb.rs`，只负责"像不像 TMDB"；D83 才把它们翻译成 `models.rs` 里的 `Episode`。

### D83 · 远程数据结构转换成本地模型

📖 [std](https://rustwiki.org/zh-CN/std/convert/trait.From.html)

**转换层的作用：把"上游的形状"挡在边界上。** `GroupEpisode` 的字段叫 `season_number` / `name`，本地 `Episode` 叫 `season` / `title`。**业务代码若直接吃 `GroupEpisode`，TMDB 一改名你就要改十个文件**；有一层 `to_local`，改动只落在这一个函数里。第 7 周的模块划分就是为这个准备的：`tmdb.rs` 认识 TMDB，`models.rs` 只认识自己。

**为什么这次不用 `From`/`Into`。** `From` 只吃一个源值，而这里除了远程条目还要补一个 `absolute`（绝对集号），**信息不够**，所以用带额外参数的普通方法更直接。**等 `absolute` 能从条目自身算出来时，再改成 `impl From<&GroupEpisode> for Episode`** 也完全自然。

**`absolute` 从哪来。** 绝对集号就是"从第 1 集一路数下来"的编号。对 TVDB 这类**分段**数据，遍历顺序（`enumerate()` 的序号 +1）就是绝对集号；对本身就是 Absolute 的分组，每集的 `episode_number` 就等于 `absolute`。

```rust
use serde::Deserialize;

// 远程形状：TMDB 的字段名
#[derive(Debug, Deserialize)]
struct GroupEpisode {
    season_number: u32,
    episode_number: u32,
    name: String,
}

// 本地形状：第 8 周起一直用的模型
#[derive(Debug, Clone, PartialEq)]
struct Episode {
    season: u32,
    episode: u32,
    title: String,
    absolute: u32,
}

impl GroupEpisode {
    // 转换层：上游字段名只出现在这里，业务代码只见本地字段
    fn to_local(&self, absolute: u32) -> Episode {
        Episode {
            season: self.season_number,
            episode: self.episode_number,
            title: self.name.clone(),
            absolute,
        }
    }
}

fn main() -> Result<(), serde_json::Error> {
    let body = r#"[
        {"season_number": 1, "episode_number": 1, "name": "The Day I Became a Shinigami"},
        {"season_number": 1, "episode_number": 2, "name": "A Shinigami's Work"}
    ]"#;
    let remote: Vec<GroupEpisode> = serde_json::from_str(body)?;

    // enumerate 从 0 开始，+1 就是 1-based 的绝对集号
    let local: Vec<Episode> = remote
        .iter()
        .enumerate()
        .map(|(i, g)| g.to_local(i as u32 + 1))
        .collect();

    for e in &local {
        println!("S{:02}E{:02} (abs={}) {}", e.season, e.episode, e.absolute, e.title);
        // S01E01 (abs=1) The Day I Became a Shinigami
        // S01E02 (abs=2) A Shinigami's Work
    }
    Ok(())
}
```

⚠️ 三条：第一，`to_local` 里写 `title: self.name` 想直接搬走，编译报 **cannot move out of self.name which is behind a shared reference**——`self` 只是借用，得 `self.name.clone()`；第二，忘了 `+1`（直接 `i as u32`），编号整体从 0 开始，**不报错**但映射全错一位；第三，`enumerate()` 给的是**段内顺序**——跨段累加绝对集号要在外层做，别每段都从 1 重新数。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 远程结构可以直接当本地模型 | 字段名和语义都可能不同，需要显式转换层 |
| 转换非得用 `From` 才算地道 | 需要额外参数时普通方法更直接，`From` 只是特例 |
| `to_local(&self)` 里能搬走 `self` 的字段 | 只能克隆或借出，`self` 是共享借用 |
| `absolute` 就是 `season` 里的集号 | 绝对集号是跨季连续编号，要单独算 |

**在 tmdb-organizer 里**：`GroupEpisode::to_local` 是第 12 周的收官——`tmdb.rs` 交出一 `Vec<Episode>`，`models.rs` 就有了与远程无关的干净数据；第 13 周 `build_plan` 只认这个本地 `Episode`，完全不碰 TMDB 的字段名。

### D84 · 复盘：不同 Order 的数据差异

📖 [TMDB](https://developer.themoviedb.org/reference/search-tv)

**三种 Order 的差别，本质是"同一个第 N 集落在哪个季"。** TVDB Order 按电视播出分段（《死神》被切成十多个 season），Absolute Order 一句不切、从 1 数到 366，DVD Order 按碟片发售重组。**同一集在三套编号里的 `(season, episode)` 往往不同，但它仍是同一集。**

**这直接决定了第 13 周的匹配策略。** 如果本地文件名按 TVDB 编、远程却拿了 Absolute 分组，**按 `season + episode` 直接匹配会大面积错配**（甚至一个都匹配不上）。所以第 13 周要先把两套编号对齐（D87 的 `HashMap`），再谈匹配。

**对比是最便宜的验证手段。** 把几套顺序的前几集并排打印，一眼就能看出"分岔"发生在哪一集。**这种"数据侦察"要尽早做**，别等写完整个映射逻辑才发现编号基准不对。

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum EpisodeOrder {
    Tvdb,
    Dvd,
    Absolute,
}

fn main() {
    // 同一批 5 集，在三套编号下的 (season, episode, title)
    let tvdb: [(u32, u32, &str); 5] = [
        (1, 1, "The Day I Became a Shinigami"),
        (1, 2, "A Shinigami's Work"),
        (1, 3, "The Older Brother's Wish"),
        (2, 1, "The Nightmare Returns"),
        (2, 2, "Beat the Invisible Enemy!"),
    ];
    // DVD 顺序把第 2、3 集调了个位置
    let dvd: [(u32, u32, &str); 5] = [
        (1, 1, "The Day I Became a Shinigami"),
        (1, 2, "The Older Brother's Wish"),
        (1, 3, "A Shinigami's Work"),
        (2, 1, "The Nightmare Returns"),
        (2, 2, "Beat the Invisible Enemy!"),
    ];
    // Absolute 顺序：不切季，集号一路数下去，季号固定为 1
    let absolute: Vec<(u32, u32, &str)> = tvdb
        .iter()
        .enumerate()
        .map(|(i, (_, _, t))| (1, i as u32 + 1, *t))
        .collect();

    for (order, list) in [
        (EpisodeOrder::Tvdb, tvdb.to_vec()),
        (EpisodeOrder::Dvd, dvd.to_vec()),
        (EpisodeOrder::Absolute, absolute),
    ] {
        println!("{order:?} Order 前 5 集：");
        for (s, e, t) in list {
            println!("  S{s:02}E{e:02}  {t}");
        }
    }
}
```

输出里能看到分岔：第 4 集在 Tvdb 顺序里是 `S02E01`、在 Absolute 顺序里却是 `S01E04`；Dvd 顺序又把第 2、3 集对调。

⚠️ 三条：第一，用 `{}` 打印枚举，编译报 **EpisodeOrder doesn't implement std::fmt::Display**；就算改成 `{:?}`，也别忘了 `#[derive(Debug)]`，否则报 **EpisodeOrder doesn't implement Debug**；第二，跨 Order 直接用 `(season, episode)` 匹配，第 4 集会一个都匹配不上，而且**不报任何错**——这就是 D87 必须做编号映射的理由；第三，算绝对集号时忘了 `+1`，整体偏移一位却不报错。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 集号在不同 Order 里只是换个季号 | 是同一集的**不同基准编号**，`(season, episode)` 可能全不一样 |
| 本地和远程天然用同一套编号 | 本地常是 TVDB 编法，远程可能是 Absolute，必须先对齐 |
| 打印枚举用 `{}` 也行 | 枚举默认没有 `Display`；用 `{:?}` 还要 `derive(Debug)` |
| 数据差异只能靠肉眼猜 | 并排打印前几集就能定位分岔点，成本极低 |

**在 tmdb-organizer 里**：`EpisodeOrder` 是第 13 周 `build_plan` 的第三个参数；D84 的结论——**TVDB 与 Absolute 在第 4 集就对不上**——正是 D87 要先建 `HashMap` 做编号映射的原因。

---

<a id="w13"></a>

## 第 13 周：核心映射，生成重命名计划

| 天   | 学 6 min                        | 写 10 min                                                    | 验收 / commit         |
| ---- | ------------------------------- | ------------------------------------------------------------ | --------------------- |
| D85  | 函数签名与职责划分              | `fn build_plan(local: &[AnimeFile], remote: &[Episode], order: EpisodeOrder) -> Vec<RenamePlan>` | 签名编译，`day85`     |
| D86  | 按 `season + episode` 匹配      | 按 `season + episode` 匹配本地和远程                         | 生成计划，`day86`     |
| D87  | `HashMap` 做映射（[书 §8.3](https://kaisery.github.io/trpl-zh-cn/ch08-03-hash-maps.html)） | 用 `HashMap<u32,(u32,u32)>` 做 Absolute→TVDB 映射            | 映射成功，`day87`     |
| D88  | 用枚举表达计划状态              | 加 `PlanStatus::Missing`                                     | 不 panic，`day88`     |
| D89  | 文件名生成与非法字符清理        | 生成 `Bleach - S01E01 - 标题.mkv`，清理非法字符              | 打印新名，`day89`     |
| D90  | 表格化输出                      | 打印计划表格，不操作文件                                     | 终端可读，`day90`     |
| D91  | 复盘：纯函数生成计划的好处      | 用假数据测试计划生成                                         | `cargo test`，`day91` |

### D85 · 函数签名与职责划分

📖 [std](https://rustwiki.org/zh-CN/std/primitive.slice.html)

**先定签名，再写实现。** `build_plan(local: &[AnimeFile], remote: &[Episode], order: EpisodeOrder) -> Vec<RenamePlan>` 这一行就把职责说清了：**收两份只读数据 + 一套编号规则，产出一份"从哪个名字改成哪个名字"的计划**。它不读文件、不写文件、不发请求——所以第 14 周的真实改名能架在它之上，而它永远可以脱网、脱文件系统地测。

**为什么用 `&[...]` 而不是 `Vec<...>`。** 传切片是"我只看看，不拿走"：调用方还留着原数据，函数也不负责释放。**参数越"松"越好**：`&[T]` 同时接受 `Vec<T>`、数组和切片；写成 `Vec<T>` 就只能吃 `Vec`，还得把所有权交出去。

**`order` 为什么要显式传进来。** 同一份 `Episode` 列表在不同 Order 下含义不同（D84），`build_plan` 不该猜，更不该自己去请求 TMDB 分辨——**把"用哪套编号"作为参数交给调用方**，函数就变成一个纯粹的映射器，第 9 周的 `--order` 参数正好喂给它。

```rust
#[derive(Debug, Clone)]
struct AnimeFile { name: String, season: u32, episode: u32 }

#[derive(Debug, Clone)]
struct Episode { season: u32, episode: u32, title: String, absolute: u32 }

#[derive(Debug, Clone, Copy, PartialEq)]
enum EpisodeOrder { Tvdb, Dvd, Absolute }

#[derive(Debug, Clone)]
struct RenamePlan { from: String, to: String, order: EpisodeOrder }

// 只负责"算"，不负责"改"：输入两串数据 + 编号方式，输出一份计划
fn build_plan(local: &[AnimeFile], remote: &[Episode], order: EpisodeOrder) -> Vec<RenamePlan> {
    // 匹配逻辑留到 D86；今天先把签名和类型钉死
    let _ = (local, remote, order);
    Vec::new()
}

fn main() {
    let local = vec![AnimeFile { name: "Bleach 01.mkv".into(), season: 1, episode: 1 }];
    let remote = vec![Episode {
        season: 1, episode: 1, title: "The Day I Became a Shinigami".into(), absolute: 1,
    }];
    println!("计划条数：{}", build_plan(&local, &remote, EpisodeOrder::Tvdb).len()); // 计划条数：0
}
```

⚠️ 三条：第一，参数写成 `Vec<AnimeFile>`（拥有所有权）却在调用处传 `&local`，编译报 **mismatched types: expected Vec<AnimeFile>, found &Vec<AnimeFile>**，提示会让你去掉那个 `&`；第二，返回类型标成 `Result<Vec<RenamePlan>, _>` 而实现返回 `Vec::new()`，报 **mismatched types: expected Result<Vec<RenamePlan>, _>, found Vec<RenamePlan>**；第三，签名里省掉 `order`，函数就只能硬编码一套编号——**参数少了不是省事，是把耦合藏进函数体**，等要加 DVD 顺序时所有调用点都得改。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 传 `Vec<T>` 比传 `&[T]` 更"完整" | `&[T]` 更通用也更省钱，调用方不必交出所有权 |
| 计划生成函数顺手读一下文件也行 | 一读文件就没法脱网测试，纯函数才是可测的前提 |
| `order` 可以让函数自己去远程分辨 | 那会把网络依赖塞进核心逻辑；调用方决定更清晰 |
| 返回 `Vec<RenamePlan>` 会漏掉"没匹配上"的信息 | 确实会，这正是 D88 用 `PlanStatus` 补上的 |

**在 tmdb-organizer 里**：`build_plan` 落在 `plan.rs`，第 14 周的 `rename.rs` 只消费它的输出；`local` 来自第 8 周目录扫描 + `parse_filename`，`remote` 来自第 12 周 `tmdb.rs` 的转换结果。

### D86 · 按 `season + episode` 匹配

📖 [std](https://rustwiki.org/zh-CN/std/iter/trait.Iterator.html)

**匹配的本质是"两个集合找共同键"。** 本地文件有 `(season, episode)`，远程条目也有 `(season, episode)`，**两个同时相等才算同一集**。最朴素的写法就是逐个查找：对每个本地文件，在远程列表里 `find` 一次。**几百集的规模下完全够用**，而且可读性最好。

**`filter_map` 一次完成"筛选 + 转换"。** 用 `map` 会产生一堆 `Option<RenamePlan>`，`collect()` 到 `Vec<RenamePlan>` 会直接失败；`filter_map` 把 `Some` 取出来、把 `None` 丢掉，**返回类型干净地就是 `Vec<RenamePlan>`**，不必再套 `flatten`。

**匹配不上的文件去哪了。** 现在是**静默丢弃**：本地 3 个文件、计划只有 2 条，少掉的那个没人告诉你。这是本周刻意留的坑，D88 用 `PlanStatus` 把它显式化——**先让它跑起来，再让它说实话**。

```rust
#[derive(Debug, Clone)]
struct AnimeFile { name: String, season: u32, episode: u32 }

#[derive(Debug, Clone)]
struct Episode { season: u32, episode: u32, title: String, absolute: u32 }

#[derive(Debug, Clone, Copy, PartialEq)]
enum EpisodeOrder { Tvdb, Dvd, Absolute }

#[derive(Debug, Clone)]
struct RenamePlan { from: String, to: String, order: EpisodeOrder }

// 匹配规则只有一条：季号和集号同时相等
fn find_remote<'a>(file: &AnimeFile, remote: &'a [Episode]) -> Option<&'a Episode> {
    remote.iter().find(|e| e.season == file.season && e.episode == file.episode)
}

fn build_plan(local: &[AnimeFile], remote: &[Episode], order: EpisodeOrder) -> Vec<RenamePlan> {
    local
        .iter()
        .filter_map(|f| {
            // filter_map：匹配上就产出一条计划，匹配不上就直接丢掉
            find_remote(f, remote).map(|ep| RenamePlan {
                from: f.name.clone(),
                to: ep.title.clone(), // 完整文件名留到 D89
                order,
            })
        })
        .collect()
}

fn main() {
    let local = vec![
        AnimeFile { name: "Bleach 01.mkv".into(), season: 1, episode: 1 },
        AnimeFile { name: "Bleach 02.mkv".into(), season: 1, episode: 2 },
        AnimeFile { name: "Bleach 99.mkv".into(), season: 1, episode: 99 },
    ];
    let remote = vec![
        Episode { season: 1, episode: 1, title: "The Day I Became a Shinigami".into(), absolute: 1 },
        Episode { season: 1, episode: 2, title: "A Shinigami's Work".into(), absolute: 2 },
    ];

    for p in build_plan(&local, &remote, EpisodeOrder::Tvdb) {
        println!("{} -> {}", p.from, p.to);
        // Bleach 01.mkv -> The Day I Became a Shinigami
        // Bleach 02.mkv -> A Shinigami's Work
    }
}
```

⚠️ 三条：第一，`map` 忘了换成 `filter_map`，`collect()` 报 **a value of type Vec<RenamePlan> cannot be built from an iterator over elements of type Option<RenamePlan>**；第二，`find_remote` 返回 `Option<&Episode>` 却没标生命周期，报 **missing lifetime specifier**；第三，匹配不上的文件被静默丢弃——**不报错**，所以"本地 3 个、计划 2 条"这种情况只能靠 D88 的状态枚举或统计输出去发现。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 双重循环太慢，一开始就得建索引 | 几百集用 `find` 完全够；优化留到 D87 再说 |
| `map` 和 `filter_map` 只差一个 `None` | `map` 会把 `None` 一起收集，`collect` 直接编译不过 |
| 匹配不上会报错 | 不会，`filter_map` 悄悄丢掉，必须自己汇报 |
| 用标题匹配更准 | 标题会改、会重名、带本地化差异，`(season, episode)` 才稳定 |

**在 tmdb-organizer 里**：`find_remote` 是 `plan.rs` 的内部函数，之后被 D87 的 `HashMap` 版本替换掉（同样的语义、更快）；`build_plan` 的对外签名一个字符都不用改。

### D87 · `HashMap` 做映射

📖 [书 §8.3](https://kaisery.github.io/trpl-zh-cn/ch08-03-hash-maps.html)

**先把"找"从 O(n) 降到 O(1)。** D86 每匹配一个文件都要扫一遍远程列表，规模一大就是几百乘几百。**用 `HashMap` 建索引：一次遍历把"绝对集号 → (季, 集)"记下来，之后每次查询是 O(1)。** `collect()` 能把 `Iterator<Item = (K, V)>` 直接收成 `HashMap`——**关键是迭代器每一项产出 `(key, value)` 元组**。

**为什么要 Absolute→TVDB 映射。** 本地文件常按绝对集号命名（`Bleach - 001.mkv`），而 TVDB 顺序里第 4 集是 `S02E01`（D84）。**先拿 TVDB 列表建索引，再用本地的绝对集号去查**，两套编号就对上了——**这是第 13 周的核心一步**。

**取用 `get` 而不是下标。** `index.get(&abs)` 返回 `Option<&(u32, u32)>`，**"远程没有这一集"是正常情况**，正好接上 D88 的 `PlanStatus::Missing`；而 `index[&abs]` 在键不存在时会 panic。

```rust
use std::collections::HashMap;

#[derive(Debug)]
struct Episode { season: u32, episode: u32, title: String, absolute: u32 }

// 用 TVDB 顺序建索引：绝对集号 -> (季, 集)
fn absolute_to_tvdb(tvdb: &[Episode]) -> HashMap<u32, (u32, u32)> {
    tvdb.iter().map(|e| (e.absolute, (e.season, e.episode))).collect()
}

fn main() {
    let tvdb = vec![
        Episode { season: 1, episode: 1, title: "The Day I Became a Shinigami".into(), absolute: 1 },
        Episode { season: 1, episode: 2, title: "A Shinigami's Work".into(), absolute: 2 },
        Episode { season: 2, episode: 1, title: "The Nightmare Returns".into(), absolute: 3 },
    ];

    let index = absolute_to_tvdb(&tvdb); // 一次建表 O(n)
    // 本地按绝对集号命名时，查一次 O(1)
    for abs in [1u32, 2, 3, 4] {
        match index.get(&abs) {
            Some((s, e)) => println!("abs {abs} -> S{s:02}E{e:02}"),
            None => println!("abs {abs} -> 远程没有这一集"),
        }
    }
    // abs 1 -> S01E01
    // abs 2 -> S01E02
    // abs 3 -> S02E01
    // abs 4 -> 远程没有这一集
}
```

⚠️ 三条：第一，忘了 `use std::collections::HashMap;`，编译报 **cannot find type HashMap in this scope**；第二，把 `get` 的返回值当元组直接点字段（`index.get(&abs).0`），报 **no field 0 on type Option<&(u32, u32)>**，得先 `match` 或 `map`；第三，同一个 `absolute` 出现两次时，`collect()` 会**后者覆盖前者且不报错**——要么建表前确认键唯一，要么用 `entry().or_insert()` 明确表达"保留第一个"。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 本地和远程集号天然一致 | 本地常是绝对集号，TVDB 是分季编号，必须先映射 |
| `collect()` 只能收成 `Vec` | 收 `HashMap` 也行，前提是每项产出 `(K, V)` 元组 |
| 键不存在时 `index[&k]` 给 `None` | 直接 panic；要 `Option` 得用 `get` |
| 重复键会报错 | 静默覆盖，想保留第一个要用 `entry().or_insert()` |

**在 tmdb-organizer 里**：`absolute_to_tvdb` 与它的逆函数都放 `plan.rs`；本地文件名走 `parse_filename` 拿到绝对集号，查完这张表就有了 TVDB 的 `(season, episode)`，随后才轮到 D89 拼新文件名。

### D88 · 用枚举表达计划状态

📖 [书 §6.1](https://kaisery.github.io/trpl-zh-cn/ch06-01-defining-an-enum.html)

**"没匹配上"不是异常，而是一种状态。** D86 里它被 `filter_map` 悄悄丢掉。**`Option` 能表达"没有"，却表达不了"是哪种没有"**——今天用枚举把每个本地文件的去向讲明白：`PlanStatus::Rename(RenamePlan)` 或者 `PlanStatus::Missing`。

**为什么把 `RenamePlan` 塞进变体，而不是用 `Option<RenamePlan>`。** `Option` 只有两种情况，**一旦以后要加 `Ambiguous`（多个远程条目撞同一集）或 `Skipped`（用户显式排除），`Option` 就装不下**；枚举加变体只需改 `match`，而编译器会**强制你把新分支补全**——D7 讲的穷尽性检查，在这里第一次产生真实的维护价值。

**别用 `unwrap`/`panic` 表达业务状态。** D75 讲过外部数据缺失是常态；`PlanStatus::Missing` 把决定权留给调用方（跳过、报错还是提示），而不是让工具崩在半路。

```rust
#[derive(Debug, Clone)]
struct AnimeFile { name: String, season: u32, episode: u32 }

#[derive(Debug, Clone)]
struct Episode { season: u32, episode: u32, title: String, absolute: u32 }

#[derive(Debug, Clone, Copy, PartialEq)]
enum EpisodeOrder { Tvdb, Dvd, Absolute }

#[derive(Debug, Clone)]
struct RenamePlan { from: String, to: String, order: EpisodeOrder }

// "这个本地文件该怎么处理"用枚举说清，而不是靠 None 悄悄消失
#[derive(Debug)]
enum PlanStatus {
    Rename(RenamePlan),
    Missing,
}

fn status_of(file: &AnimeFile, remote: &[Episode], order: EpisodeOrder) -> PlanStatus {
    match remote
        .iter()
        .find(|e| e.season == file.season && e.episode == file.episode)
    {
        Some(ep) => PlanStatus::Rename(RenamePlan {
            from: file.name.clone(),
            to: ep.title.clone(),
            order,
        }),
        // 找不到不是异常，只是一种状态：少了远程数据而已
        None => PlanStatus::Missing,
    }
}

fn main() {
    let local = vec![
        AnimeFile { name: "Bleach 01.mkv".into(), season: 1, episode: 1 },
        AnimeFile { name: "Bleach 99.mkv".into(), season: 1, episode: 99 },
    ];
    let remote = vec![Episode {
        season: 1, episode: 1, title: "The Day I Became a Shinigami".into(), absolute: 1,
    }];

    for f in &local {
        match status_of(f, &remote, EpisodeOrder::Tvdb) {
            PlanStatus::Rename(p) => println!("[改名] {} -> {}", p.from, p.to),
            // [改名] Bleach 01.mkv -> The Day I Became a Shinigami
            PlanStatus::Missing => println!("[缺失] {} 在远程找不到对应集，跳过", f.name),
            // [缺失] Bleach 99.mkv 在远程找不到对应集，跳过
        }
    }
}
```

⚠️ 三条：第一，`match` 只写了 `Rename` 分支，编译报 **non-exhaustive patterns: &PlanStatus::Missing not covered**；第二，忘了 `#[derive(Debug)]` 却用 `{:?}` 打印 `PlanStatus`，报 **PlanStatus doesn't implement std::fmt::Debug**；第三，"缺失"该跳过还是该报错**不由 `build_plan` 决定**——它只如实汇报状态，策略留在第 14 周的 CLI 层，加 `--strict` 就是多写一个 `match` 分支，不必改核心逻辑。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `Option` 够用，不必上枚举 | 多于两种情况时 `Option` 表达不了，枚举才好扩展 |
| 找不到对应集就该 `panic` 或 `unwrap` | 缺失是正常状态，应作为枚举变体向上汇报 |
| 枚举加变体会悄悄影响别的分支 | 所有 `match` 会立刻编译失败，逼你补全 |
| 状态判断该塞进 `build_plan` | 计划照旧生成，缺失状态单独汇报，职责更清楚 |

**在 tmdb-organizer 里**：`PlanStatus` 定义在 `plan.rs`，第 14 周的 dry-run 表格会把它渲染成"跳过 / 待改名"两列；这也是 D77 说的"降级要有明确信号"的落地。

### D89 · 文件名生成与非法字符清理

📖 [std](https://rustwiki.org/zh-CN/std/primitive.str.html)

**文件名生成是一条"模板流水线"。** `剧名 - S01E01 - 标题.ext` 每段都有来源：剧名来自第 8 周拿到的系列名、`S01E01` 来自远程 `(season, episode)`（D83 转换后）、标题来自远程 `name`、扩展名来自原文件。**抽成一个纯函数 `build_filename`**，每段就能单独测。

**为什么必须清理非法字符。** Windows 文件名禁止九个字符（反斜杠、斜杠、冒号、星号、问号、引号、尖括号、竖线等），而 TMDB 标题里 `:`、`?`、`/` 都很常见（`Squad 13: The Battle/Aftermath`）。**不清理的后果不是当场报错，而是 `fs::rename` 跑到那一集时才突然失败**——排查成本远高于提前替换。

**还有两个隐蔽规则。** 文件名**不能以空格或点结尾**（`Trailing dots...` 要裁掉）；替换要"一对一"别"删除"：**`replace("/", "")` 会让 `A/B` 变成 `AB`，两个不同标题可能撞名**，换成下划线更安全。

```rust
// Windows 文件名禁用字符：\ / : * ? " < > |
fn sanitize(title: &str) -> String {
    title
        .chars()
        .map(|c| if "\\/:*?\"<>|".contains(c) { '_' } else { c })
        .collect::<String>()
        // Windows 还不允文件名以空格或点结尾
        .trim_end_matches([' ', '.'])
        .to_string()
}

fn build_filename(series: &str, season: u32, episode: u32, title: &str, ext: &str) -> String {
    format!("{series} - S{season:02}E{episode:02} - {}.{ext}", sanitize(title))
}

fn main() {
    println!("{}", build_filename("Bleach", 1, 1, "The Day I Became a Shinigami", "mkv"));
    // Bleach - S01E01 - The Day I Became a Shinigami.mkv
    println!("{}", build_filename("Bleach", 1, 6, "Squad 13: The Battle/Aftermath", "mkv"));
    // Bleach - S01E06 - Squad 13_ The Battle_Aftermath.mkv
    println!("{}", build_filename("Bleach", 1, 7, "Trailing dots... ", "mkv"));
    // Bleach - S01E07 - Trailing dots.mkv
}
```

⚠️ 三条：第一，`chars().map(...)` 少了 `.collect()`，编译报 **expected String, found Map<Chars<'_>, ...>**——迭代器适配器不会自己变回 `String`；第二，清理时漏掉某个字符（比如忘了竖线），**编译和运行都不报错**，直到某集改名失败才暴露；第三，忘了裁掉结尾的空格或点，Windows 上会被系统直接拒绝（提示文件名语法不正确），而同样的名字在 Linux 上却能建成——**跨平台差异**只能靠统一的 `sanitize` 抹平。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 标题不干净最多是难看 | Windows 会直接拒绝，`fs::rename` 运行到那一集才失败 |
| 用 `replace` 删掉非法字符最省事 | 删除会改变内容长度，可能让两个标题撞名，替换成下划线更稳 |
| 结尾的点无所谓 | Windows 不允许文件名以空格或点结尾 |
| 清理一次就够，扩展名也一起洗 | 扩展名来自本地文件、本来就是合法的，只该洗标题段 |

**在 tmdb-organizer 里**：`build_filename` 与 `sanitize` 放 `plan.rs`（或单开 `rename.rs` 里的纯工具函数）；D86 里那句 `to: ep.title.clone()` 到这一天被换成 `build_filename(series, ep.season, ep.episode, &ep.title, ext)`，`RenamePlan.to` 才成为真正可直接使用的目标名。

### D90 · 表格化输出

📖 [std](https://rustwiki.org/zh-CN/std/fmt/index.html)

**表格化输出 = 把"数据"变成"可核对的东西"。** dry-run 的价值在于**动手前让人看一眼**，所以信息要成列对齐：原文件名、新文件名、用哪套顺序。**`{:<24}` 是左对齐 + 最小宽度**，内容超宽不截断、只是把表格挤歪。

**为什么先实现打印、再实现改名。** 第 14 周才动文件；**先把计划打出来，比"先改名、再发现改错"便宜太多**。这正是 D85 让 `build_plan` 保持纯函数的回报——打印只是它的一个消费者。

**表头用 ASCII 更稳。** 中文是全角字符，`{:<24}` 按**字符数**补空格、不算显示宽度，所以中文列在终端里会明显歪。**要么表头用英文，要么自己算显示宽度**（本教程不引入额外的表格库，把复杂度留在能接受的范围）。

```rust
#[derive(Debug, Clone, Copy, PartialEq)]
enum EpisodeOrder { Tvdb, Dvd, Absolute }

#[derive(Debug)]
struct RenamePlan { from: String, to: String, order: EpisodeOrder }

// 只打印，不碰文件系统：dry-run 的第一步（第 14 周才真正改名）
fn print_plan(plans: &[RenamePlan]) {
    println!("{:<24} {:<26} {}", "FROM", "TO", "ORDER");
    println!("{}", "-".repeat(64));
    for p in plans {
        println!("{:<24} {:<26} {:?}", p.from, p.to, p.order);
    }
}

fn main() {
    let plans = vec![
        RenamePlan { from: "Bleach 01.mkv".into(), to: "Bleach - S01E01.mkv".into(), order: EpisodeOrder::Tvdb },
        RenamePlan { from: "Bleach 02.mkv".into(), to: "Bleach - S01E02.mkv".into(), order: EpisodeOrder::Tvdb },
        RenamePlan { from: "Bleach 03.mkv".into(), to: "Bleach - S01E03.mkv".into(), order: EpisodeOrder::Absolute },
    ];
    print_plan(&plans);
}
```

输出：

```text
FROM                     TO                         ORDER
----------------------------------------------------------------
Bleach 01.mkv            Bleach - S01E01.mkv        Tvdb
Bleach 02.mkv            Bleach - S01E02.mkv        Tvdb
Bleach 03.mkv            Bleach - S01E03.mkv        Absolute
```

⚠️ 三条：第一，用 `{}` 打印枚举列，编译报 **EpisodeOrder doesn't implement std::fmt::Display**，得改成 `{:?}` 并确保 `derive(Debug)`；第二，宽度写太小（如 `{:<4}`）**不会截断也不报错**，只会把长文件名挤出去、表格失去可读性；第三，中文表头能编译能运行，但终端里列会错位——**这类"看起来对、其实歪"的问题只能靠肉眼看输出发现**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `{:<24}` 按显示宽度补空格 | 按**字符数**补，中文是全角，列会对不齐 |
| 宽度写小了会自动截断 | 不会，只是把后面的列挤歪 |
| dry-run 就是"不改文件" | 还要给人足够信息核对；表格化就是这一步的产出 |
| 打印属于"临时调试代码" | 它是产品的一部分，第 14 周会直接复用它 |

**在 tmdb-organizer 里**：`print_plan` 落在 `plan.rs` 或 `rename.rs`；第 14 周 `--dry-run` 走的就是它，非 dry-run 时才在打印之后调用 `fs::rename`——**先看表，再动手**。

### D91 · 复盘：纯函数生成计划的好处

📖 [书 §11.1](https://kaisery.github.io/trpl-zh-cn/ch11-01-writing-tests.html)

**纯函数是"可测"的前提。** `build_plan` 只吃两串数据、吐一份计划，**不碰文件系统、不发网络**，所以测试里手搓 `AnimeFile` 和 `Episode` 就能跑——**既不需要 `.env` 里的 key，也不需要真实的剧集目录**。这正是第 12 周坚持"先转成本地模型"的回报。

**测试要盯"边界"而不是"正常路"。** 三个测试分别覆盖：匹配上的会生成计划、没匹配上的不出现在计划里、`order` 原样带进计划。**"没匹配上被跳过"这条最值钱**——它把 D86 的静默行为固化成可观察的断言；将来 D88 的 `PlanStatus` 重构时，测试会告诉你行为有没有变。

**测完再改，改完再测。** 第 14 周的 `rename.rs`、第 15 周的 `anyhow` 重构都会碰这里；**有测试兜底，重构才敢动手**（D46 的 `#[cfg(test)]` 到这一天终于真正派上用场）。

```rust
#[derive(Debug, Clone, PartialEq)]
struct AnimeFile { name: String, season: u32, episode: u32 }

#[derive(Debug, Clone, PartialEq)]
struct Episode { season: u32, episode: u32, title: String, absolute: u32 }

#[derive(Debug, Clone, Copy, PartialEq)]
enum EpisodeOrder { Tvdb, Dvd, Absolute }

#[derive(Debug, Clone, PartialEq)]
struct RenamePlan { from: String, to: String, order: EpisodeOrder }

// 纯函数：不碰文件系统、不发网络请求，所以能用假数据直接测
fn build_plan(local: &[AnimeFile], remote: &[Episode], order: EpisodeOrder) -> Vec<RenamePlan> {
    local
        .iter()
        .filter_map(|f| {
            remote
                .iter()
                .find(|e| e.season == f.season && e.episode == f.episode)
                .map(|ep| RenamePlan { from: f.name.clone(), to: ep.title.clone(), order })
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    fn 本地() -> Vec<AnimeFile> {
        vec![AnimeFile { name: "Bleach 01.mkv".into(), season: 1, episode: 1 }]
    }

    fn 远程() -> Vec<Episode> {
        vec![Episode {
            season: 1, episode: 1, title: "The Day I Became a Shinigami".into(), absolute: 1,
        }]
    }

    #[test]
    fn 匹配上的文件生成计划() {
        let plan = build_plan(&本地(), &远程(), EpisodeOrder::Tvdb);
        assert_eq!(plan.len(), 1);
        assert_eq!(plan[0].to, "The Day I Became a Shinigami");
    }

    #[test]
    fn 没匹配上的文件被跳过() {
        let local = vec![AnimeFile { name: "Bleach 99.mkv".into(), season: 1, episode: 99 }];
        assert!(build_plan(&local, &远程(), EpisodeOrder::Tvdb).is_empty());
    }

    #[test]
    fn 顺序原样带进计划() {
        let plan = build_plan(&本地(), &远程(), EpisodeOrder::Dvd);
        assert_eq!(plan[0].order, EpisodeOrder::Dvd);
    }
}
```

`cargo test` 会报 `3 passed`。

⚠️ 三条：第一，`assert_eq!` 比较 `EpisodeOrder` 而该类型没 `derive(PartialEq)`，编译报 **binary operation == cannot be applied to type EpisodeOrder**；第二，`assert_eq!` 还要 `Debug`（失败时要打印两侧的值），只写 `PartialEq` 会报 **EpisodeOrder doesn't implement Debug**，所以 `Debug, PartialEq` 要成对出现；第三，测试模块里忘了 `use super::*;`，报 **cannot find function build_plan in this scope**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 测试计划生成必须先准备好真目录 | 纯函数用假数据就能测，和文件系统无关 |
| 测"正常路径"就够了 | 边界（没匹配上、空远程列表）才是真正会出问题的地方 |
| `assert_eq!` 只需要 `PartialEq` | 还需要 `Debug`，否则编译不过 |
| 重构时容易悄悄改坏行为 | 有断言兜底；行为一变测试立刻红 |

**在 tmdb-organizer 里**：这些测试落在 `plan.rs` 的 `#[cfg(test)] mod tests`；第 14 周加 `fs::rename`、第 15 周换成 `anyhow` 之后，**这组测试保证核心映射逻辑始终没变**——第 13 周到此收官，项目从"能解析出远程集数"升级成"能算出一份可核对、可测试的重命名计划"。

---

<a id="w14"></a>

## 第 14 周：文件操作、dry-run、安全确认

| 天   | 学 6 min                                  | 写 10 min                               | 验收 / commit         |
| ---- | ----------------------------------------- | --------------------------------------- | --------------------- |
| D92  | `fs::create_dir_all`（[std](https://rustwiki.org/zh-CN/std/fs/fn.create_dir_all.html)） | 创建目标季文件夹                        | 目录出现，`day92`     |
| D93  | `fs::rename` 重命名（[std](https://rustwiki.org/zh-CN/std/fs/fn.rename.html)） | 重命名单个文件                          | 文件改名，`day93`     |
| D94  | dry-run 与副作用隔离                      | `--dry-run` 只打印不执行                | 不操作文件，`day94`   |
| D95  | `stdin` 读取确认输入（[std](https://rustwiki.org/zh-CN/std/io/struct.Stdin.html)） | 无 `--yes` 时要求输入 `y/n`             | 可取消，`day95`       |
| D96  | 目标冲突检测与跳过策略                    | 目标已存在则跳过或加后缀                | 不覆盖，`day96`       |
| D97  | 真实文件的重命名集成测试                  | 用 tempdir 造文件，运行计划，断言重命名 | `cargo test`，`day97` |
| D98  | 复盘：不可逆操作的防护                    | 在复制的小目录实测                      | 成功整理，`day98`     |

### D92 · `fs::create_dir_all` 创建目标季文件夹

📖 [std](https://rustwiki.org/zh-CN/std/fs/fn.create_dir_all.html)

**为什么是 `create_dir_all` 而不是 `create_dir`。** 目标季文件夹通常是 `./sample/Season 01` 这样的**多层**路径，中间任何一层不存在都会失败。`create_dir` 只建最后一级、不补父目录；`create_dir_all` 把缺失的父目录一并补齐。整理目录时你不想先判断"上一层在不在"——**让一次调用覆盖所有前置条件**，代码里就少了一整棵 `if`。

**它天生幂等，这一点比"方便"更重要。** 目录已存在时 `create_dir_all` 返回 `Ok` 而不是报错，于是"确保这个目录存在"可以从一句声明直接执行，不用先 `if !dir.exists()`。**少一次前置检查，就少一个 TOCTOU 窗口**：从 `exists()` 返回 false 到真正创建之间，别的进程可能刚把同名目录建好或删掉，你的判断已经过期。

**代价是它每次都要问一遍文件系统。** 所以别在重命名循环里对同一个季文件夹反复调用——**进入循环前建一次，把 `PathBuf` 拿着用**，循环里只做重命名。

```rust
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

// 目标季文件夹：season 1 -> "Season 01"
fn ensure_season_dir(root: &Path, season: u32) -> io::Result<PathBuf> {
    let dir = root.join(format!("Season {season:02}"));
    fs::create_dir_all(&dir)?; // 已存在不报错，缺的父目录也一并补齐
    Ok(dir)
}

fn main() -> io::Result<()> {
    let root = std::env::temp_dir().join("bleach-demo"); // 演示只用临时/示例路径
    let _ = fs::remove_dir_all(&root);
    let s1 = ensure_season_dir(&root, 1)?;
    let s1_again = ensure_season_dir(&root, 1)?; // 第二次调用：目录已存在，静默通过
    println!("{} 存在 = {}", s1.display(), s1.is_dir()); // ...\Season 01 存在 = true
    println!("两次结果相同 = {}", s1 == s1_again);         // 两次结果相同 = true
    fs::remove_dir_all(&root)?;
    Ok(())
}
```

⚠️ 三条：第一，在返回 `()` 的 `main` 里直接写 `fs::create_dir_all(&dir)?`，编译报 **the ? operator can only be used in a function that returns Result or Option**（E0277），得把签名改成 `fn main() -> io::Result<()>`；第二，把 `create_dir_all` 误写成 `create_dir`，父目录缺失时**编译通过、运行时才炸**，报 **No such file or directory (os error 2)**（Windows 上是 os error 3）；第三，**别对用户在命令行给的原目录做创建或清理**——只对"目标季文件夹"这种自己算出来的路径动手。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 目录已存在时 `create_dir_all` 会报错 | 返回 `Ok`，天生幂等，可以直接"确保存在" |
| `create_dir` 和 `create_dir_all` 只差个名字 | `create_dir` 不补父目录，多层路径必然失败 |
| 先 `exists()` 再建更稳妥 | 两步之间有竞态；直接 `create_dir_all` 更简单也更稳 |
| 每处理一个文件都该确保一次目录 | 建一次就够，循环里重复调用只是白跑系统调用 |

**在 tmdb-organizer 里**：`ensure_season_dir` 落在 `rename.rs`，`execute` 在进入重命名循环前调用一次，返回的 `PathBuf` 留给后面拼目标路径；**示例里的根一律是 `./sample/` 或临时目录，绝不拿用户的原目录当创建/删除的目标**。

### D93 · `fs::rename` 重命名单个文件

📖 [std](https://rustwiki.org/zh-CN/std/fs/fn.rename.html)

**rename 是"改名"，不是"复制"。** 同一块文件系统内它只改目录项、**不搬数据**，所以几十 GB 的一季几乎瞬时完成，也不会有"复制到一半断电留下半个文件"的问题。但它**不跨文件系统**：源和目标不在同一个盘或挂载点时会直接失败——"整理到另一个盘"只能用 `copy` + `remove_file`，rename 没有 `create_dir_all` 那种帮你补齐的耐心。

**它会静默覆盖已存在的目标，这是最危险的一点。** Windows 和 Unix 上 `fs::rename` 都是**先删掉同名目标再改名**，成功返回 `Ok`、不留任何提示。如果新名恰好撞上另一集，那个文件就被无声替换了——**rename 之前必须先做冲突检测（D96），而不是事后补救**。

**为什么它属于"不可逆操作"。** 一次 `rename` 没有撤销键：文件换到新路径后，旧路径就不存在了，除非你自己记着 `from`。这也是整周反复强调"先 `--dry-run` 看计划"的原因——**改一次名的代价是常数，悔一次棋的代价是无穷**。

```rust
use std::fs;
use std::io;
use std::path::Path;

// 全项目唯一真正改文件的地方：一次只做一次 rename
fn rename_one(from: &Path, to: &Path) -> io::Result<()> {
    fs::rename(from, to)?; // 同盘是"改名"，不搬数据
    Ok(())
}

fn main() -> io::Result<()> {
    let dir = std::env::temp_dir().join("bleach-d93");
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir)?;
    let from = dir.join("Bleach 01.mkv");
    fs::write(&from, "video")?;
    let to = dir.join("Bleach - S01E01.mkv");

    rename_one(&from, &to)?;
    println!("旧名还在 = {}", from.exists()); // 旧名还在 = false
    println!("新名存在 = {}", to.exists());   // 新名存在 = true
    fs::remove_dir_all(&dir)?;
    Ok(())
}
```

⚠️ 三条：第一，源文件不存在时运行报 **No such file or directory (os error 2)**——`rename` 不会替你核对"这个文件到底在不在计划里"，名字拼错就当场失败；第二，**目标已存在不报错**，实测目标内容被静默替换（返回 `Ok`、无任何警告），保护只能由调用方自己写；第三，跨盘移动报 **The system cannot move the file to a different disk drive. (os error 17)**——这是系统限制不是 bug，重试多少次都一样。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| rename 会复制文件内容 | 同盘只改目录项，几乎瞬时；跨盘则直接失败 |
| 目标存在时 rename 会报错 | 会静默覆盖，返回 `Ok`，不给你任何提示 |
| rename 失败重试一下就好 | 失败多半是系统级限制（跨盘、权限），重试无用 |
| 中途失败会自动回滚 | 没有事务；前面几条已改名，只能靠 dry-run 预演 |

**在 tmdb-organizer 里**：`rename.rs` 里那句 `fs::rename(&p.from, &p.to)?` 是全局唯一的危险动作；D96 的 `exec_one` 会在它前面加一道"目标已存在就跳过"的闸门——**危险动作用一处代码写完，保护也就只需要守住这一处**。

### D94 · dry-run 与副作用隔离

📖 [std](https://rustwiki.org/zh-CN/std/fs/index.html)

**dry-run 的本质是把一件事拆成"算"和"做"两半。** 计划（`Vec<RenamePlan>`）是"算"的产物，它不依赖任何文件是否真的存在；"做"才去调 `fs::rename`。所以 dry-run 不是"少做一点"，而是**根本不去调用有副作用的函数**——D85 坚持 `build_plan` 是纯函数，回报就在这里。

**副作用必须收口到一个地方。** 项目里所有"会改变外界"的动作（写文件、改文件名、发网络请求）只允许出现在 `execute` / `apply` 这一个函数里；`build_plan`、`print_plan`、`parse_filename` 全是纯的。**一旦副作用散落各处，dry-run 就再也保证不了"真的没动"**，因为总有一处会漏判 `dry_run`。

**`if dry_run` 只判断一次。** 把它写在函数入口，而不是每个 `rename` 前面：入口一处、出口一处，**读代码的人一眼就能确认所有路径都被覆盖**。写在多处就变成"改一处忘一处"，而漏判的后果是"以为在干跑、其实改了名"。

```rust
use std::fs;
use std::io;

#[derive(Debug, Clone, Copy)]
enum EpisodeOrder { Tvdb, Dvd, Absolute }

#[derive(Debug)]
struct RenamePlan { from: String, to: String, order: EpisodeOrder }

// 唯一入口：dry_run 为真时只打印，绝不触碰文件系统
fn apply(plans: &[RenamePlan], dry_run: bool) -> io::Result<()> {
    for p in plans {
        if dry_run {
            println!("[dry-run] {} -> {} ({:?})", p.from, p.to, p.order);
        } else {
            fs::rename(&p.from, &p.to)?; // 只有这一行会改变磁盘
        }
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let dir = std::env::temp_dir().join("bleach-d94"); // 只用临时/示例路径
    fs::create_dir_all(&dir)?;
    fs::write(dir.join("Bleach 01.mkv"), "x")?;

    let plans = [RenamePlan {
        from: dir.join("Bleach 01.mkv").display().to_string(),
        to: dir.join("Bleach - S01E01.mkv").display().to_string(),
        order: EpisodeOrder::Tvdb,
    }];
    apply(&plans, true)?; // 干跑：文件原地不动
    println!("干跑后原名还在 = {}", dir.join("Bleach 01.mkv").exists()); // true
    Ok(())
}
```

⚠️ 三条：第一，`apply(&plans, ...)` 传成 `apply(plans, ...)`（`Vec` 而非切片），编译报 **mismatched types: expected &[RenamePlan], found Vec<RenamePlan>**，提示会建议加 `&`；第二，漏掉 `?`（写成 `apply(&plans, true);`）返回值被丢弃，警告 **unused Result that must be used**——D103 打开 `-D warnings` 后它会直接让编译失败；第三，把 `if dry_run` 写进某个分支而非入口，**干跑照样会改文件**，编译和测试都不报，只能靠"副作用只出现在一个函数里"这条纪律防住。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| dry-run 是"少做一点事" | 是**不调用**有副作用的函数，一行都不碰文件系统 |
| 每个 `fs::rename` 前都判一次 `dry_run` 更保险 | 判得越多越容易漏；入口判一次，覆盖范围才可验证 |
| 判断散在几处，测试能发现问题 | 测试只覆盖走过的路径，漏判的分支照样静默通过 |
| dry-run 只对用户有意义 | 它也是开发纪律：逼你把副作用集中，才谈得上可测 |

**在 tmdb-organizer 里**：`--dry-run` 走的就是 `rename.rs` 里 `apply` 的打印分支，非 dry-run 才落到那唯一一行 `fs::rename`；D90 的 `print_plan` 在本周被它复用——**先在 `./sample/` 上干跑看表，确认无误再谈执行**。

### D95 · `stdin` 读取确认输入

📖 [std](https://rustwiki.org/zh-CN/std/io/struct.Stdin.html)

**为什么不可逆操作要加一道人工确认。** 程序再小心也会遇到"计划看着对、其实全错"的情况（剧集命名不规范、用户点错目录）。**把最后一步的决定权交回给人**：没加 `--yes` 时先问一句，回车即取消。默认动作选"取消"（`[y/N]` 而不是 `[Y/n]`），是因为**误按的代价不对称**——多问一次只是烦，误改一次是丢文件。

**`print!` 之后必须手动 flush。** `println!` 带换行会刷新缓冲，`print!` 不会：提示词会**卡在缓冲区里**，程序已经在等输入、屏幕上却什么都没有，看起来像卡死。所以写完提示词要补一句 `io::stdout().flush()?`。

**`read_line` 一次读一整行，连换行一起给你。** 它是**追加**到 `String` 末尾（不是替换），返回值是**读到的字节数**而不是内容；读到的内容形如 `"y\r\n"`。所以比较前必须 `trim()`，否则用户敲了 `y` 也会被当成取消。

```rust
use std::io::{self, Write};

// 无 --yes 时要求用户敲 y 才继续；回车或 EOF 一律视为取消
fn confirm(prompt: &str) -> io::Result<bool> {
    print!("{prompt} [y/N] ");
    io::stdout().flush()?; // 不 flush，提示词可能一直不显示
    let mut line = String::new();
    io::stdin().read_line(&mut line)?; // 读到 "y\r\n"，含换行
    Ok(matches!(line.trim().to_lowercase().as_str(), "y" | "yes"))
}

fn main() -> io::Result<()> {
    if confirm("确定要重命名《死神》共 366 个文件吗？")? {
        println!("开始执行");
    } else {
        println!("已取消，未改任何文件");
    }
    Ok(())
}
```

⚠️ 三条：第一，少写 `use std::io::Write`，`io::stdout().flush()` 报 **no method named flush found for struct Stdout in the current scope**（E0599，提示要让 `Write` 进入作用域）；第二，忘了 `trim()`，`"y\r\n"` 和 `"y"` 永远不相等——**用户敲了 y 也会被当成取消**（实测读到的正是 3 字节 `"y\r\n"`）；第三，把 `read_line` 的返回值当内容用（`let ans = io::stdin().read_line(&mut line)?;` 再拿 `ans` 比字符串），报 **mismatched types: expected usize, found &str**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `read_line` 会替换缓冲区内容 | 它是**追加**；反复调用要自己 `line.clear()` |
| 提示词写完就会显示 | `print!` 不带换行，不 flush 就可能看不到 |
| 读进来的是 `"y"` | 是 `"y\r\n"`，不 `trim()` 判断永远为假 |
| 管道/重定向下确认还有意义 | 读到 EOF 时返回空串，被判为取消，正好是安全默认 |

**在 tmdb-organizer 里**：`confirm` 落在 `cli.rs` 或 `rename.rs`，`execute` 在非 dry-run 且没有 `--yes` 时调它一次；`--yes` 是给脚本和 CI 用的——**交互确认给真人兜底，`--yes` 给自动化放行**。

### D96 · 目标冲突检测与跳过策略

📖 [std](https://rustwiki.org/zh-CN/std/path/struct.Path.html)

**为什么必须自己查冲突。** D93 已经确认：`fs::rename` 遇到已存在的目标**不报错，直接覆盖**。所以"目标名已经被占用"这件事**没有系统兜底**，只能由你在动手前用 `to.exists()` 查出来。这不是防御性编程，而是补齐标准库故意留给你的一步——Rust 不替你决定"撞名了该怎么办"。

**撞名有两种处理：跳过，或改名让位。** 跳过（`SkippedExists`）**最安全**，信息不丢、只是这次没整理到，适合当默认行为；加 `(1)`、`(2)` 后缀（`free_name`）不丢文件也能整理完，代价是文件名变长、和 TMDB 的规范名不再一一对应。**默认跳过、需要时显式开启让位**，比反过来更容易解释。

**`exists()` 有它的盲区。** 它跟随符号链接，**指向不存在目标的坏链接会返回 false**；而且从 `exists()` 到 `rename` 之间依然存在竞态。所以 `exists()` 是**降低概率**而不是**保证安全**——真正的最终保障，还是 dry-run 里人眼看一遍。

```rust
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[derive(Debug, PartialEq)]
enum ExecStatus { Done, SkippedExists }

// 目标已存在时绝不覆盖：跳过并把决定权交回上层
fn exec_one(from: &Path, to: &Path) -> io::Result<ExecStatus> {
    if to.exists() {
        return Ok(ExecStatus::SkippedExists);
    }
    fs::rename(from, to)?;
    Ok(ExecStatus::Done)
}

// 备选策略：撞名就加 " (1)"、" (2)" 后缀，直到腾出一个空位
fn free_name(to: &Path) -> PathBuf {
    if !to.exists() { return to.to_path_buf(); }
    let stem = to.file_stem().unwrap().to_string_lossy().into_owned();
    let ext = to.extension().map(|e| e.to_string_lossy().into_owned()).unwrap_or_default();
    let parent = to.parent().unwrap();
    let mut n = 1;
    loop {
        let cand = parent.join(format!("{stem} ({n}).{ext}"));
        if !cand.exists() { return cand; }
        n += 1;
    }
}

fn main() -> io::Result<()> {
    let dir = std::env::temp_dir().join("bleach-d96"); // 只用临时/示例路径
    let _ = fs::remove_dir_all(&dir);
    fs::create_dir_all(&dir)?;
    fs::write(dir.join("Bleach 01.mkv"), "new")?;
    fs::write(dir.join("Bleach - S01E01.mkv"), "already here")?; // 目标名已被占用

    let (from, to) = (dir.join("Bleach 01.mkv"), dir.join("Bleach - S01E01.mkv"));
    println!("{:?}", exec_one(&from, &to)?);          // SkippedExists
    println!("原名还在 = {}", from.exists());          // 原名还在 = true
    println!("让位后 = {}", free_name(&to).display());  // ...Bleach - S01E01 (1).mkv
    fs::remove_dir_all(&dir)?;
    Ok(())
}
```

⚠️ 三条：第一，早返回漏了 `return`（写成 `if to.exists() { Ok(ExecStatus::SkippedExists) }`），编译报 **mismatched types: expected (), found Result<ExecStatus, _>**（E0308）——分支得凑出 `()`，表达式的值被丢掉了；第二，`to.file_stem().unwrap()` 在名字以点开头（如 `.gitkeep`）时**运行期 panic**，处理点文件时必须先判 `None`；第三，**别用"跳过"掩盖真问题**：一季里大半都是 `SkippedExists`，通常说明目标名生成规则撞车了，该去查 `build_filename` 而不是默默跳过。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| rename 会帮你挡住撞名 | 它静默覆盖，`to.exists()` 得你自己查 |
| `exists()` 为真就一定是普通文件 | 目录也算；要区分用 `is_file()` / `is_dir()` |
| 查过 `exists()` 就万无一失 | 检查与 rename 之间仍有竞态，只是把概率降到极低 |
| 跳过等于失败 | 跳过是**安全失败**：原文件没动，信息没丢，还能重跑 |

**在 tmdb-organizer 里**：`ExecStatus` 和 `exec_one` 落在 `rename.rs`，`execute` 的循环里除 `Done` 外多一个 `SkippedExists` 分支走 `eprintln!`（stderr，不污染干跑表格）；冲突策略由 CLI 决定——**默认跳过，想拿到"保留原名让位"的整洁输出就切到 `free_name`**。

### D97 · 真实文件的重命名集成测试

📖 [docs](https://docs.rs/tempfile/latest/tempfile/)

**为什么要测"真实文件"而不只是纯函数。** D91 的测试只覆盖 `build_plan`（不碰文件系统）。但这一周新增的风险全在 IO 里：`create_dir_all` 的路径拼对没有、`rename` 的源和目标是不是同一个目录、冲突检测真的拦住没有——**这些用假数据永远测不出来**。集成测试就是**在临时目录里造出真实文件，跑一遍执行函数，再断言文件名真的变了**。

**为什么必须用 `tempdir`。** 测试**绝不能**去读写死的 `./sample/`：换台机器、进 CI 就找不到，真跑起来还会污染工作区。`tempdir()` 每次给一个**唯一**目录，`TempDir` 一出作用域就被 `Drop` 掉——**失败的测试也不会在磁盘上留垃圾**。

**断言要盯两件事：新名出现、旧名消失。** 只断言新名存在不够（文件可能是被复制而不是被改名），必须同时断言**旧名不存在**，才算真的"移动"。再加一条"目标已存在时跳过"，就把 D96 的保护也钉进了测试。

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn renames_files_in_real_dir() {
        let dir = tempdir().unwrap(); // 唯一临时目录，出作用域自动删除
        fs::write(dir.path().join("Bleach 01.mkv"), "x").unwrap();
        fs::write(dir.path().join("Bleach 02.mkv"), "x").unwrap();

        let plans = vec![
            RenamePlan { from: "Bleach 01.mkv".into(), to: "Bleach - S01E01.mkv".into(), order: EpisodeOrder::Tvdb },
            RenamePlan { from: "Bleach 02.mkv".into(), to: "Bleach - S01E02.mkv".into(), order: EpisodeOrder::Tvdb },
        ];
        for p in &plans { // 注意是 &plans，否则后面不能再借用
            exec_one(&dir.path().join(&p.from), &dir.path().join(&p.to)).unwrap();
        }

        assert!(dir.path().join("Bleach - S01E01.mkv").exists()); // 新名出现
        assert!(!dir.path().join("Bleach 01.mkv").exists());      // 旧名消失
        assert_eq!(plans.len(), 2);
    }

    #[test]
    fn target_conflict_is_skipped() {
        let dir = tempdir().unwrap();
        fs::write(dir.path().join("Bleach 01.mkv"), "new").unwrap();
        fs::write(dir.path().join("taken.mkv"), "old").unwrap();
        let st = exec_one(&dir.path().join("Bleach 01.mkv"), &dir.path().join("taken.mkv")).unwrap();
        assert_eq!(st, ExecStatus::SkippedExists); // 没有覆盖
        assert!(dir.path().join("Bleach 01.mkv").exists());
    }
}
```

⚠️ 三条：第一，`for p in plans`（不带 `&`）会消费掉 `plans`，之后再 `plans.len()` 报 **borrow of moved value: plans**（E0382，附注 value borrowed here after move）；第二，`assert_eq!(st, ExecStatus::SkippedExists)` 但枚举没 `derive(PartialEq)`，报 **binary operation == cannot be applied to type ExecStatus**（E0369），没 `derive(Debug)` 另报 **ExecStatus doesn't implement Debug**（E0277）；第三，断言里忘了拼 `dir.path()`（写成 `Path::new("Bleach - S01E01.mkv").exists()`），**它查的是当前工作目录而不是临时目录**，测试恒为假却完全不报错。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 纯函数测过就不用测 IO 了 | 路径拼错、跨目录改名、冲突漏判，只有真实文件测得出 |
| 用固定目录（`./sample/`）测更省事 | 换机器/进 CI 就不存在，还会污染工作区；必须用 `tempdir` |
| 断言新名存在就够了 | 要同时断言旧名消失，否则"复制"也能骗过测试 |
| 测试失败会留下垃圾目录 | `TempDir` 靠 `Drop` 清理，失败也一样删 |

**在 tmdb-organizer 里**：这一组测试和 D55 的 `scan_dir` 测试同放在 `src/main.rs` 的 `#[cfg(test)] mod tests` 里（第 15 周拆模块后跟随 `rename.rs` 走）；它和 `tests/parser_test.rs` 分工——**单元测试管纯逻辑，这里管真实文件**。

### D98 · 复盘：不可逆操作的防护

📖 [std](https://rustwiki.org/zh-CN/std/io/enum.ErrorKind.html)

**防护的本质是"给不可逆操作加前置闸门"。** 这一周把几道闸门拼到一起：**目标只在 `./sample/` 或临时目录**（D92）、**默认 `--dry-run` 先看计划**（D94）、**再来人工确认**（D95）、**冲突跳过**（D96）、**有测试兜底**（D97）。它们不是并列的可选项，而是**层层收窄**：前一道没拦住的，由后一道兜住。

**为什么"默认安全"是好默认。** `--dry-run` 是人显式关掉的、`--yes` 是人显式加上的——**要出错得连着踩两个显式开关**。反过来设计（默认就改、`--dry-run` 才停）则把"安全"变成需要用户记得开启的选项，忘记一次就丢数据。安全默认的原则是：**危险动作必须被"明确意图"解锁**，而不是被"明确意愿"阻止。

**区分错误种类，才能给出对的提示。** 同样是 `io::Error`，`ErrorKind::NotFound`（源不见了）和 `PermissionDenied`（没权限）该说的话完全不同；`AlreadyExists`（目标占用）则根本不该当成错误往上抛——它是**预期内的状态**，D96 用 `ExecStatus` 表达掉了。**先按 `kind()` 分类，再决定"报错、跳过还是重试"**。

```rust
use std::fs;
use std::io;
use std::path::Path;

#[derive(Debug, PartialEq)]
enum ExecStatus { Done, SkippedExists }

fn exec_one(from: &Path, to: &Path) -> io::Result<ExecStatus> {
    if to.exists() { return Ok(ExecStatus::SkippedExists); }
    fs::rename(from, to)?;
    Ok(ExecStatus::Done)
}

// 三道闸门合成一个入口：dry-run -> 人工确认 -> 冲突跳过
fn execute(plans: &[RenamePlan], dry_run: bool, yes: bool) -> io::Result<()> {
    if dry_run {
        for p in plans { println!("[dry-run] {} -> {}", p.from, p.to); }
        return Ok(()); // 干跑到此为止，绝不往下走
    }
    if !yes && !confirm(&format!("将重命名 {} 个文件，继续？", plans.len()))? {
        println!("已取消，未改动任何文件");
        return Ok(());
    }
    for p in plans {
        match exec_one(Path::new(&p.from), Path::new(&p.to))? {
            ExecStatus::Done => println!("[ok] {} -> {}", p.from, p.to),
            ExecStatus::SkippedExists => eprintln!("[skip] 目标已存在：{}", p.to),
        }
    }
    Ok(())
}
```

（`RenamePlan` 见 D85，`confirm` 见 D95——`execute` 是 `rename.rs` 对外唯一暴露的动作。）

⚠️ 三条：第一，dry-run 分支写完忘了 `return`，**干跑打印完会继续往下走**：先弹确认、再真改名，最该拦住的那道闸门变成摆设；第二，把 `--yes` 的默认值写成 `true`，等于给脚本开了后门，**交互确认永远不触发**——`yes` 必须默认 false、显式加才生效；第三，**`ErrorKind` 不是万能的**：Windows 上"文件被占用"常报成别的 kind，别把逻辑写成死等某一种 kind，拿不准就把路径和 `e` 一起打印出来。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 有 dry-run 就够了，不用确认 | 计划本身可能全错；人眼确认拦的是"计划对但意图错" |
| 安全默认是"默认开启安全选项" | 是**危险动作必须被显式解锁**（`--yes`），忘掉等于安全 |
| `io::Error` 只有一种，统一处理即可 | `kind()` 区分 NotFound / PermissionDenied 等，提示方式不同 |
| 目标被占用应当报错退出 | 它是预期内的状态，用 `ExecStatus` 表达、跳过更好 |

**在 tmdb-organizer 里**：`execute` 把 D92-D97 的成果串成一条链，`main` 只负责 `let args = cli::Args::parse();` 再调它；**实测永远在 `./sample/`（或 `%TEMP%` 下的副本）里做**——D98 的验收就是拿一份复制的小目录跑一遍，确认"先干跑、后执行、冲突跳过"三步都对。

---

<a id="w15"></a>

## 第 15 周：完善、模块化、README、发布准备

| 天   | 学 6 min                                | 写 10 min                                        | 验收 / commit      |
| ---- | --------------------------------------- | ------------------------------------------------ | ------------------ |
| D99  | `anyhow` 简化错误（[docs](https://docs.rs/anyhow/latest/anyhow/)） | `cargo add anyhow`，`main -> anyhow::Result<()>` | 错误简化，`day99`  |
| D100 | 日志输出与 `--verbose` 分级             | 加 `--verbose`，用 `eprintln!` 或 tracing        | 调试信息，`day100` |
| D101 | 模块拆分与依赖方向（[书 §7.5](https://kaisery.github.io/trpl-zh-cn/ch07-05-separating-modules-into-different-files.html)） | 拆 `cli.rs`、`tmdb.rs`、`plan.rs`、`rename.rs`   | 编译通过，`day101` |
| D102 | README 的安装/配置/用法结构             | 安装、配置 API key、用法、示例                   | 别人能跑，`day102` |
| D103 | `cargo fmt` 与 `clippy`（[附录 D](https://kaisery.github.io/trpl-zh-cn/appendix-04-useful-development-tools.html)） | `cargo fmt && cargo clippy -- -D warnings`       | 无警告，`day103`   |
| D104 | 边界测试与集成测试补齐                  | `cargo test` 全绿，补边界测试                    | 全绿，`day104`     |
| D105 | release 构建与优化配置（[书 §14.1](https://kaisery.github.io/trpl-zh-cn/ch14-01-release-profiles.html)） | `cargo build --release`，真实目录实测            | 可用，`day105`     |

### D99 · `anyhow` 简化错误

📖 [docs](https://docs.rs/anyhow/latest/anyhow/)

**为什么需要 anyhow。** 第 4 周你手写了 `ParseError` 枚举、为它 impl `std::error::Error`、再补 `From` 转换；第 10 周换成了 `Box<dyn Error>`。走到这一步会发现：**错误处理写了一大堆，却没给用户带来多少价值**——用户只想知道"哪一步失败了"。`anyhow::Error` 是一个**能把任何 `std::error::Error` 装进去的容器**，配 `?` 自动转换，**一行 `From` 都不用写**。

**它最值钱的是"错误链"。** `context` / `with_context` 给错误**贴一层"我在干什么"的说明**，底层原因原样留在链上。打印时上下文在前、根因在后（`{e:#}`，或 `main` 返回 `Err` 时自动格式化），**既有"哪一步"又有"为什么"**——这正是手写枚举最难兼顾的部分。

**它不是万能的，边界要划清。** `anyhow::Error` **只适合当"最终的错误"用，不适合当返回值来区分处理**：调用方没法按类型 `match`（要 `downcast`，很别扭）。所以**库代码（`parser.rs`、`plan.rs` 这种要被测试和复用的逻辑）继续用具体错误类型，只有 `main` 和 CLI 层用 anyhow**——这是 D46/D47"边界在哪、错误类型就长在哪"的延续。

```rust
use anyhow::{bail, Context, Result};

// 读配置：任何一步出错都带上"在干什么"的上下文
fn load_key(path: &str) -> Result<String> {
    let text = std::fs::read_to_string(path).with_context(|| format!("读取 {path} 失败"))?;
    let key = text.trim().strip_prefix("TMDB_API_KEY=").unwrap_or("");
    if key.is_empty() {
        bail!("{path} 里没有 TMDB_API_KEY"); // bail! 等价于 return Err(anyhow!(..))
    }
    Ok(key.to_string())
}

fn main() -> Result<()> {
    let dir = std::env::temp_dir().join("bleach-d99");
    std::fs::create_dir_all(&dir)?;
    let env = dir.join(".env");
    std::fs::write(&env, "TMDB_API_KEY=abc123\n")?;

    println!("读到 key = {}", load_key(&env.display().to_string())?); // 读到 key = abc123

    let e = load_key("./sample/not-here.env").unwrap_err();
    println!("错误链：{e:#}"); // 读取 ./sample/not-here.env 失败: ...
    Ok(())
}
```

⚠️ 三条：第一，漏写 `use anyhow::Context`，`.with_context(...)` 报 **no method named with_context found for enum Result<T, E> in the current scope**（E0599，提示要让 `Context` 进入作用域）；第二，`main` 返回 `anyhow::Result<()>` 后若真返回了 `Err`，**进程退出码是 1**、stderr 打出 `Error: 上下文` 加一段 `Caused by:` 的根因——**别在 `main` 里再手动 `unwrap()`**，那会丢掉格式化好的错误链；第三，**别把 `anyhow::Error` 塞进库函数签名**（比如 `parse_filename`），那样调用方再也没法按类型分支，测试里也不好断定具体错误。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| anyhow 会丢掉"是哪种错误" | 链上仍保留根因类型，可用 `downcast` 取回；只是默认不利 |
| 用了 anyhow 就不需要 `Display`/`Error` | 它装的就是实现了 `Error` 的类型，标准库那套还是地基 |
| 全项目都该换成 anyhow | 库边界换掉会毁掉可分支性；只该在 `main`/CLI 层收口 |
| `bail!` 只是简写的 `return` | 它构造的是 `anyhow::Error`，还能带格式化参数 |

**在 tmdb-organizer 里**：`main` 从 `fn main() -> std::io::Result<()>` 改成 `fn main() -> anyhow::Result<()>`，`load_key`（配置读取）用 `context` 给出"是读 `.env` 失败还是 key 缺失"；`parser.rs` / `plan.rs` 的签名不动——**anyhow 收在门口，不渗进核心逻辑**。

### D100 · 日志输出与 `--verbose` 分级

📖 [std](https://rustwiki.org/zh-CN/std/macro.eprintln.html)

**日志和结果走两条不同的流。** `println!` 进 stdout、`eprintln!` 进 stderr。干跑的重命名表格是**产品输出**（用户可能 `> plan.txt` 存起来），调试信息是**过程输出**（只该给人看）。**一旦日志用 `println!`，重定向时就会混进表格里**：`cargo run -- ./sample --dry-run > plan.txt` 得到的文件会掺进一堆"运行中…"。

**`--verbose` 是计数，不是开关。** 第 9 周用 `ArgAction::Count` 把它做成 `-v`/`-vv`：**0 只报结果，1 报步骤，2 报细节**。分级的价值在于——**默认输出保持干净，出问题时用户可以自己调高级别**，而不需要你专门发一个"调试版"。

**级别判断集中到一个 `log` 函数。** 每个调用点都写 `if args.verbose >= 1` 会把判断散得到处都是；抽成 `log(verbose, level, msg)` 后，**"什么级别显示什么"只在一个地方定义**，将来想换成 `tracing` 也只是换这一个函数（本教程不引入额外依赖，把复杂度留在可控范围）。

```rust
// 日志级别来自 --verbose 的 Count：0=只报结果，1=报步骤，2=报细节
fn log(verbose: u8, level: u8, msg: &str) {
    if verbose >= level {
        eprintln!("[{level}] {msg}"); // 走 stderr：不污染 stdout 上的干跑表格
    }
}

fn main() {
    let verbose: u8 = 2; // 相当于命令行给了 -vv
    log(verbose, 1, "开始扫描《死神》剧集目录");
    log(verbose, 2, "发现 Bleach 01.mkv，解析为 S01E01");
    log(0, 1, "这条在 -v 为 0 时不会出现"); // 0 >= 1 为假，静默
    println!("计划 1 条"); // 结果始终走 stdout
}
```

输出（`[n]` 开头的都来自 stderr）：

```text
[1] 开始扫描《死神》剧集目录
[2] 发现 Bleach 01.mkv，解析为 S01E01
计划 1 条
```

⚠️ 三条：第一，把 `format!(...)` 直接传给收 `&str` 的 `log`，报 **mismatched types: expected &str, found String**——要么加 `&`，要么让 `log` 收 `impl AsRef<str>`；第二，**日志用 `println!`**，重定向到文件时和结果混在一起，机器读不了、人也不好搜；第三，别拿 `Count` 的 `u8` 去减（写成 `args.verbose - 1`）——**`u8` 减到 0 以下会 panic**（attempt to subtract with overflow），级别比较一律用 `>=`。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 日志和结果都用 `println!` 更省事 | 重定向时会混进数据；日志应走 stderr |
| `--verbose` 是个 bool 开关 | 它是 `Count`，`-vv` 得到 2，能表达多级 |
| 日志越详细越好 | 默认必须干净；详细度由用户显式调高 |
| 日志是临时调试代码，随时删 | 它是产品的一部分，`--verbose` 就是给它的稳定入口 |

**在 tmdb-organizer 里**：`log` 落在 `cli.rs`（或 `main.rs` 的辅助函数），`verbose` 字段就是第 9 周 D61 那个 `Count` 参数；`tmdb.rs` 抓到的响应体预览、`plan.rs` 的匹配细节都用 `level = 2` 打印——**默认安静，`-vv` 时才有第一手证据**。

### D101 · 模块拆分与依赖方向

📖 [书 §7.5](https://kaisery.github.io/trpl-zh-cn/ch07-05-separating-modules-into-different-files.html)

**为什么现在才拆。** 前 14 周所有代码挤在 `main.rs` 是刻意的：**逻辑没定型就拆文件，只会得到一堆改了名字的函数**。到第 15 周，职责已经稳定——扫描（`parser`）、取远程（`tmdb`）、生成计划（`plan`）、执行改名（`rename`）、CLI 参数（`cli`）、共享类型（`models`），此时拆，**每个文件都对应一个能一句话说清的职责**。

**依赖方向必须是单向的。** `main` 依赖所有模块；`cli`/`parser`/`plan`/`rename`/`tmdb` 只依赖 `models`（共享类型），**彼此之间尽量不互相依赖**。这样 `plan` 能脱离文件和网络单独测（D91）、`parser` 能脱离 TMDB 单独测（D47）。**一旦出现环形依赖（`plan` 回头 `use crate::main`），可测性立刻崩塌**。

**拆分的判据是"谁需要知道多少"。** `main` 只需要知道"依次调用哪几个函数"，所以它最短；`rename` 知道怎么改文件、却不知道计划怎么来的；`tmdb` 知道 HTTP 和 JSON、却不知道文件名长什么样。**每个模块对外暴露的函数越少，耦合越低**——D85 那条"先定签名"的纪律，在这里升级成"先定模块的公开接口"。

```rust
// src/main.rs：只负责"装配"，不含任何业务逻辑
mod cli;      // 命令行参数（Args）
mod models;   // AnimeFile / Episode / RenamePlan / EpisodeOrder
mod parser;   // 扫描目录 + 解析文件名
mod plan;     // 纯函数：build_plan
mod rename;   // 唯一改文件的地方：execute
mod tmdb;     // HTTP + JSON -> Vec<Episode>

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let args = cli::Args::parse();                             // main -> cli
    let local = parser::scan(&args.dir)?;                      // main -> parser
    let remote = tmdb::fetch_episodes("Bleach", args.order)?;  // main -> tmdb
    let plans = plan::build_plan(&local, &remote, args.order); // main -> plan
    rename::execute(&plans, args.dry_run)?;                    // main -> rename
    Ok(())
}
```

依赖方向一图看清：

```text
main ──> cli / parser / tmdb / plan / rename ──> models
plan 不认识文件系统，rename 不认识 TMDB，tmdb 不认识文件名
```

⚠️ 三条：第一，`EpisodeOrder` 忘了 `derive(Copy)`，`args.order` 传给 `tmdb` 后再传给 `plan` 报 **use of moved value: args.order**（E0382，附注 which does not implement the Copy trait）——**枚举当配置值来回传时一律 derive `Copy`**；第二，模块文件没在 `main.rs` 里写 `mod xxx;`，文件存在也不参与编译，用的时候报 **failed to resolve: use of unresolved module or unlinked crate xxx**；第三，跨模块用 `crate::` 绝对路径（`use crate::models::AnimeFile;`）而不是 `super::super::...`——**后者一挪文件就全断，前者与文件位置无关**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 文件拆得越细越好 | 拆到"职责一句话说不清"就是负担；按职责拆而非按行数 |
| 拆文件后要重写代码 | 只是移动 + 补 `mod`/`use`，逻辑一行不改 |
| 模块之间互相 `use` 很正常 | 互相依赖会毁掉可测性；共享类型下沉到 `models` |
| `mod` 声明只影响编译顺序 | 没声明的文件根本不编译；声明是模块的唯一起点 |

**在 tmdb-organizer 里**：拆完就是 README 末尾"最终文件结构"那张图——`src/` 下 `main.rs` 只留装配，`cli.rs` 放 `Args`、`models.rs` 放共享类型、`parser.rs`/`plan.rs`/`rename.rs`/`tmdb.rs` 各管一段；**`tests/parser_test.rs` 继续按第 7 周的方式引用解析逻辑**。

### D102 · README 的安装/配置/用法结构

📖 [docs](https://docs.rs/dotenvy/latest/dotenvy/)

**README 是给"第一次打开仓库的人"看的。** 他自己没有 `.env`、不知道要装什么、也不知道该敲哪条命令。结构上就三块：**安装**（怎么拿到二进制：`cargo build --release` 或下载 Release）、**配置**（需要什么环境变量、填到哪里）、**用法**（能直接复制的示例命令加一段示例输出）。**顺序不能反**——先告诉人怎么装上，再谈配置，最后才是用法。

**配置一节必须"缺了就报清楚"。** 最容易踩的坑是：用户忘了填 key，程序跑起来只吐一句 `missing key` 就退出。**错误里要包含"缺什么、去哪填、怎么填"**，这正是 D99 `context` 的用武之地。**另外，永远不要把 key 本身打印出来**——日志、README、错误信息里都不能出现真 key。

**示例命令要能真的跑通。** README 里写 `tmdb-organizer ./sample --dry-run`，如果参数名改过而文档没跟上，用户复制过去就报错、信任立刻归零。D62 的做法在这里收尾：**用 `Args::try_parse_from` 把 README 里的每条命令喂给解析器**，让文档变成可执行的契约；示例路径一律用 `./sample/`，**不鼓励读者上来就对真实目录跑**。

```rust
use anyhow::{Context, Result};

// README「配置」一节的要求：从环境变量读 TMDB_API_KEY（本地由 .env 注入）
fn api_key() -> Result<String> {
    std::env::var("TMDB_API_KEY").context("未设置 TMDB_API_KEY，请在 .env 里填好后重试")
}

fn main() {
    match api_key() {
        Ok(_) => println!("已读到 API key"),  // 只说"读到了"，绝不打印 key 本身
        Err(e) => println!("配置错误：{e}"),  // 配置错误：未设置 TMDB_API_KEY，...
    }
}
```

⚠️ 三条：第一，README 里的示例命令和 `Args` 对不上（比如文档写成 `--dryrun`），用户复制就报 **unexpected argument '--dryrun' found**——**加一条 `try_parse_from` 测试就能在 CI 里拦住**；第二，把 API key 写进 README 或打进日志，**等于公开泄露**，示例一律用占位符 `your_api_key_here`；第三，`.env` 必须进 `.gitignore`：**仓库里只提交 `.env.example`**，这条要写进"配置"一节，否则下一个人 clone 完一脸茫然。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| README 写得越全越好 | 只写三块：安装、配置、用法；其余放文档或注释 |
| 示例命令过时了用户会自己猜 | 参数名错一位就报错；要用测试把示例钉住 |
| 报"缺少 key"就够清楚了 | 要说明缺哪个、去哪填、格式如何，最好给一行示例 |
| `.env.example` 可有可无 | 它是配置一节的入口，没有它新人不知道要填什么 |

**在 tmdb-organizer 里**：README 的"配置"一节对应 `dotenvy::dotenv()` 加载 `.env`（第 10 周）与这里的 `api_key()`；"用法"一节的示例命令就是 D62 那组 `try_parse_from` 用例里的字符串——**文档和测试引用同一份真相**。

### D103 · `cargo fmt` 与 `clippy -- -D warnings`

📖 [附录 D](https://kaisery.github.io/trpl-zh-cn/appendix-04-useful-development-tools.html)

**fmt 消除"风格争论"，clippy 消除"绕弯路"。** `cargo fmt` 是**无脑**的：把代码重排成官方风格（缩进、换行、括号位置），不接受争辩——**省下的是 review 里辩论空格的时间**。`cargo clippy` 是**有判断的**：它找"能编译、但写法绕"的地方，比如比长度用 `len() == 0` 而不是 `is_empty()`、手写 `match Some/None` 而不是 `.map()`。

**为什么要在 CI 里加 `-- -D warnings`。** clippy 默认只**警告**，警告看多了就没人看。加 `-D warnings` 把警告升级成错误，**"有警告就构建失败"**，团队才真的会被迫修。代价是 clippy 升级可能带来新 lint，需要随手修或显式 `allow`——所以**本地先跑 `cargo fmt && cargo clippy -- -D warnings`，再提 PR**，别让 CI 替你发现。

**它修的是"读起来更直白"，不是"跑得更快"。** 大多数 lint 对性能没有影响，价值在于**让意图一眼可见**：`is_empty()` 比 `len() == 0` 更明确地表达"我关心的是空不空，不是长度"。**别为了讨好 clippy 把代码改得更绕**——真觉得某条 lint 不合适，用 `#[allow(clippy::xxx)]` 加一句注释说明理由，比硬凑好。

```rust
// 这两处是 clippy 会点名的典型：改完更直白，不是更快
fn is_empty_dir(names: &[String]) -> bool {
    names.is_empty() // clippy::len_zero：别写 names.len() == 0
}

fn title_len(ep: &Option<String>) -> Option<usize> {
    ep.as_ref().map(|t| t.len()) // clippy::manual_map：别手写 match Some/None
}

fn main() {
    let v: Vec<String> = Vec::new();
    println!("{} {:?}", is_empty_dir(&v), title_len(&Some("Bleach".to_string())));
    // true Some(6)
}
```

改之前，clippy 给的是这三条（加了 `-- -D warnings` 之后直接变成 error）：

```text
warning: unneeded `return` statement                       (clippy::needless_return)
warning: length comparison to zero                         (clippy::len_zero)
  help: using `is_empty` is clearer and more explicit
warning: manual implementation of `Option::map`            (clippy::manual_map)
  help: try: `ep.as_ref().map(|t| t.len())`
```

⚠️ 三条：第一，只跑 `cargo clippy` 不加 `-- -D warnings`，CI 照样绿——**警告等于没提**；第二，clippy 的提示写的是"更清楚的写法"（例如 **using is_empty is clearer and more explicit**），**不是"必须这样"**，不适用的 lint 应显式 `#[allow]` 并写清原因，而不是硬改；第三，`cargo fmt` 会重排你精心对齐的代码（超长的 `println!` 会被拆成多行，`cargo fmt -- --check` 会先报出来），**它是无条件的**——当成"提交前必做"，而不是"可选美化"。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| `cargo build` 通过就没有问题 | clippy 会发现能编译但绕的写法；fmt 会改风格 |
| clippy 的 lint 都是性能问题 | 多数是表达清晰度；极少数才真的影响性能 |
| 本地不用跑，等 CI 报 | CI 只告诉你"不行"，定位和修还得本地做，往返更慢 |
| fmt 会自动改你的逻辑 | 它只重排空白与换行，语义一行不变 |

**在 tmdb-organizer 里**：`rename.rs` 里 `if !to.exists() { ... }` 这类紧凑写法会被 fmt 规范化，`plan.rs` 里手写的 `match` 会被 clippy 提醒改用 `.map()`；第 16 周的 GitHub Actions（D109）就是 `cargo fmt --check` 加 `cargo clippy -- -D warnings`——**这两条命令从今天起是每次提交的门槛**。

### D104 · 边界测试与集成测试补齐

📖 [书 §11.3](https://kaisery.github.io/trpl-zh-cn/ch11-03-test-organization.html)

**"正常路径"测过了，剩下全是边界。** 现有测试覆盖了"两个文件都能匹配上"，但真实目录里的麻烦恰恰在少数情况：**空目录、一集都没匹配上、标题带非法字符、同名撞车**。边界用例的价值不是"多跑几条"，而是**把"我们决定怎么处理它"写下来**——将来有人改 `build_plan`，这些断言会立刻告诉他"你动了契约"。

**单元测试和集成测试各有分工。** `#[cfg(test)] mod tests` 测**单个函数**（`build_plan`、`sanitize`、`status_of`），快、不碰磁盘；`tests/` 目录与临时目录里的测试测**真实文件**（造目录、跑 `execute`、断言文件名），慢但覆盖真实风险。**两者都要有**：只有单元测试会漏掉"路径拼错"，只有集成测试则失败时定位不清（这正是 D47"测试公共 API"与 D91"纯函数好测"的合流）。

**`assert!` 要能一眼看出在测什么。** `assert!(matches!(status, PlanStatus::Missing))` 比一串 `!=` 清楚得多；**每条用例只测一个行为**，失败时不必猜是哪个断言炸的。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_input_gives_empty_plan() {
        // 空目录不是错误：计划为空即可，别 panic
        assert!(build_plan(&[], &[], EpisodeOrder::Tvdb).is_empty());
    }

    #[test]
    fn unmatched_file_is_missing() {
        let f = AnimeFile { name: "Bleach 99.mkv".into(), season: 1, episode: 99 };
        assert!(matches!(status_of(&f, &[], EpisodeOrder::Tvdb), PlanStatus::Missing));
    }

    #[test]
    fn illegal_chars_are_sanitized() {
        // 标题里的 : 和 / 都要被替换，结尾的点要裁掉
        assert_eq!(sanitize("Squad 13: Battle/Aftermath"), "Squad 13_ Battle_Aftermath");
        assert_eq!(sanitize("Trailing dots... "), "Trailing dots");
    }
}
```

⚠️ 三条：第一，`assert_eq!` 要求两侧都实现 `Debug` 和 `PartialEq`：枚举忘了 `derive(PartialEq)` 报 **E0369**（binary operation == cannot be applied to type X），忘了 `derive(Debug)` 报 **E0277**（X doesn't implement Debug）——D97 已经栽过一次；第二，**"空输入"这类用例最容易被当成没意义而省略**，但空目录正是新用户第一次跑工具的常见输入，它决定工具是"友好退出"还是"panic"；第三，集成测试里的断言**别写相对路径**（`Path::new("Bleach - S01E01.mkv")` 查的是当前工作目录），必须从 `dir.path()` 拼起来。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 测试数量够多就安全 | 关键在于覆盖边界，不在条数；空输入/撞名才是雷区 |
| 边界情况很少见，可以不测 | 新用户第一次跑就是空目录；少见的才是崩溃来源 |
| 单元测试和集成测试重复劳动 | 一个测逻辑、一个测真实 IO，各自能测出对方测不出的 |
| 断言写得越复杂越严谨 | 一条用例一个行为、一眼看懂，失败时定位才快 |

**在 tmdb-organizer 里**：这组用例和 D91 的测试同放 `plan.rs` 的 `#[cfg(test)] mod tests`，真实文件的测试跟 D97 放一起；测试文件与最终文件结构里的 `tests/parser_test.rs` 并列——**`cargo test` 全绿，是第 15 周的验收线**。

### D105 · release 构建与优化配置

📖 [书 §14.1](https://kaisery.github.io/trpl-zh-cn/ch14-01-release-profiles.html)

**debug 与 release 是两套配置，不是"同一份代码的快慢之分"。** `cargo run` 用 `dev` profile：**不优化、保留 debug 断言、编译快**；`cargo build --release` 用 `release` profile：**开 `opt-level=3`、关 debug 断言、编译慢、跑得快**。**别拿 debug 版测性能**——同一段代码差几十倍很正常，而且**行为本身就不一样**（D2 讲过：debug 下整数溢出 panic，release 下静默回绕）。

**"代码里能区分构建类型"是个实用开关。** `cfg!(debug_assertions)` 在 debug 下为 true、release 下为 false。**用它来定默认日志级别**：本地开发默认 `-v` 方便看流程，发布版默认安静、用户需要时再加参数——**同一份源码，本地和发布版行为不同，但各自都合理**。

**`[profile.release]` 是"用编译时间换产物"的地方。** 默认 release 已经开了优化，想要更小更快可以再加几项：`lto = true`（跨 crate 链接期优化）、`codegen-units = 1`（牺牲并行编译换更优代码）、`strip = true`（去掉符号表，体积明显变小）、可选 `panic = "abort"`（去掉 unwind 支持，再小一点）。**代价是编译变慢**，而且这些只影响 `--release`，日常 `cargo run` 完全不受影响。

```rust
fn main() {
    // debug_assertions 只在 dev 构建里开着，release 下关闭
    let build = if cfg!(debug_assertions) { "debug" } else { "release" };
    let verbose = if cfg!(debug_assertions) { 1 } else { 0 }; // 发布版默认安静
    println!("构建 = {build}"); // cargo run 打印 debug，--release 打印 release

    // 冷启动工作：《死神》共 366 集，编号求和
    let total: u64 = (1..=366u64).sum();
    println!("默认 verbose = {verbose}，366 集编号合计 = {total}"); // 67161
}
```

`Cargo.toml` 里对应的优化配置：

```toml
[profile.release]
codegen-units = 1
lto = true
strip = true
panic = "abort"
```

⚠️ 三条：第一，`strip = true` 之后**运行时的 panic 回溯会缺少符号**（只剩地址），要排查问题得临时去掉它再复现；第二，`panic = "abort"` 会让 `catch_unwind` **不再生效**，个别平台与测试框架下还需要额外处理——**不确定就先别加**，`lto` 加 `strip` 已经能拿到大部分收益；第三，**别把 debug 下的表现当基准**：release 关掉了整数溢出检查和 `debug_assert!`，一个"本地怎么都不崩"的 bug 可能只在发布版复现——这正是 D104 那批边界测试必须留在 CI 的原因。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| release 只是"更快"，行为一样 | 溢出检查、`debug_assert!` 都关了，行为可能不同 |
| 加了 `lto` 就是全面变好 | 编译时间明显变长；小项目收益有限，按需开 |
| `strip` 只影响体积 | 也影响回溯可读性；调试时要临时关掉 |
| `--release` 的产物默认不依赖配置 | 它由 `[profile.release]` 决定，可被 `Cargo.toml` 改 |

**在 tmdb-organizer 里**：`cargo build --release` 产出的 `target/release/tmdb-organizer` 就是"安装"一节让用户下载或本地构建的那个二进制；按 D100 的分级约定，可以用 `cfg!(debug_assertions)` 把发布版的默认 `verbose` 定为 0——**发布版默认安静，只在 `./sample/` 上跑，确认无误再指向真实目录**。

---

<a id="w16"></a>

## 第 16 周：毕业、GitHub、Release、写“不再入门”

| 天   | 学 6 min                                | 写 10 min                                                    | 验收 / commit                  |
| ---- | --------------------------------------- | ------------------------------------------------------------ | ------------------------------ |
| D106 | 真实数据实测与问题记录方法              | 用工具整理复制版《死神》，记录问题                           | 问题清单，`day106`             |
| D107 | 缺陷定位与最小化修复                    | 修最烦的问题 1                                               | commit，`day107`               |
| D108 | 回归验证与提交规范                      | 修问题 2                                                     | commit，`day108`               |
| D109 | GitHub Actions 工作流（[docs](https://docs.github.com/zh/actions)） | GitHub Actions：test、clippy、fmt                            | push 自动跑，`day109`          |
| D110 | LICENSE 与 `Cargo.toml` 元数据（[书 §14.2](https://kaisery.github.io/trpl-zh-cn/ch14-02-publishing-to-crates-io.html)） | 加 LICENSE，补 `Cargo.toml`                                  | `cargo package` 通过，`day110` |
| D111 | 打 tag 与 GitHub Release 流程（[docs](https://docs.github.com/zh/repositories/releasing-projects-on-github/managing-releases-in-a-repository)） | 发 GitHub Release `v1.0.0`                                   | Release 页面，`day111`         |
| D112 | 项目复盘与产出总结                      | README 顶部写：“我不再入门 Rust。我用 Rust 写了 tmdb-organizer。” | 复盘 16 周，`day112`           |

### D106 · 真实数据实测与问题记录方法

📖 [GitHub 文档：Issues](https://docs.github.com/zh/issues)

**为什么必须在复制版上测。** 有两个原因。一是**不可逆**：366 个文件一旦真被改了名，做种、播放列表、字幕匹配全断，而重命名**没有撤销命令**——你手里唯一能回退的东西是干跑输出那份计划。二是**可重复**：修复前后的两次运行必须在**同一份输入**上做，否则你分不清"输出变了"到底是代码变了还是输入变了。所以先用 `cp -r` 把真实剧集复制成 `./bleach-test/`，原件只读。

**记录格式比"赶紧修"更重要。** 每条问题只写四行：**现象**（看到什么）、**复现**（哪条命令）、**期望**、**实际**。这四行把"感觉不太对"变成可核对的事实，也让别人（和明天的你）能独立复现。**先记录再修复**是纪律，理由看这张表：

| 做法 | 结果 |
| ---- | ---- |
| 先记录再修复 | 现象被钉死，修完能逐条对照；改动范围可控 |
| 边看边改 | 改到第三处时已记不清第一处现象，也无法确认究竟修对了没有 |

**清单按严重度排，不按出现顺序。** 会改错的（把 S01E02 改成 S01E03）> 会漏处理的（剧场版被吞掉）> 只是难看的（`·` 被换成 `_`）。另外别忘了 D100 的分工：**表格走 stdout、日志走 stderr**，存证时要**分别重定向**——用 `2>&1` 把两者混成一个文件，之后就没法逐行对比了。

```bash
# 1. 复制一份再动手：原件只读，所有测试只在副本上做
cp -r ./bleach-original ./bleach-test

# 2. 先干跑，把"修之前"的输出存成基准；-vv 的日志单独存，不混进表格
cargo run --release -- ./bleach-test --dry-run -vv > issues/run-01.txt 2> issues/run-01.log

# 3. 问题清单：每条四行（现象 / 复现 / 期望 / 实际）
#    P1 集号整体偏移：Bleach 02.mkv 算成 S01E03（期望 S01E02）
#       复现：cargo run --release -- ./bleach-test --dry-run -vv | head -20
#    P2 剧场版被当成第 366 集
#    P3 标题里的「·」被 sanitize 成「_」，不致命但难看

# 4. 看一眼副本现状，确认原件一个字没动
ls -1 ./bleach-test | head -5
```

⚠️ 三条：第一，**忘了 `--dry-run` 直接对真实目录跑就没有退路**——先造副本、先干跑，这一步不能省；第二，把日志和表格混存进一个文件后，diff 满屏噪音、机器也读不了，**stdout 与 stderr 必须分开存**；第三，路径含空格或中文时忘了加引号，报错是 **No such file or directory (os error 2)**，看着像"文件名没匹配上"，其实是 shell 把路径切开了。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 出问题改回来就行 | 重命名不可逆；唯一能回退的是干跑输出那份计划 |
| 干跑和真跑结果一定一样 | 真跑会受文件已存在等因素影响，个别冲突只在真跑时暴露 |
| 发现问题要马上动手改 | 先记录：记录没留下，"改对了"就无从对照 |
| 问题清单只是备忘 | 它是 D107、D108 两天修复工作的唯一输入 |

**在 tmdb-organizer 里**：`./bleach-test/` 就当真实数据，跑的是 D105 的 release 二进制；`issues/run-01.txt` 与 `run-01.log` 就是后面两天回归验证的基准——**没有它，D108 的回归无从谈起**。

### D107 · 缺陷定位与最小化修复

📖 [书 §11.1 编写测试](https://kaisery.github.io/trpl-zh-cn/ch11-01-writing-tests.html)

**先最小化复现，再动代码。** 在真实目录上"改一版跑一遍"，你会陷入"改了、看着对了、其实只是另一个文件碰巧正常"的循环，因为**输入太大、反馈太慢**。最小化的判据很硬：**输入缩到只剩一个字符串或一个文件**，且不碰磁盘、不发网络。到这一步，缺陷就变成一行 `assert`，`cargo test` 一秒给答案。

**定位顺序：从数据往代码倒推。** 先看 `-vv` 的中间态——**解析出来的 season/episode 到底是几**。若解析阶段就错了（`Bleach 02.mkv` 得到 `S01E03`），根因在 `parser.rs`；若解析对了而匹配结果不对，才轮到 `plan.rs`。**别从报错行开始猜**：报错行显示的是症状，数据是在上游被算错的。

**一次只改一处，改动要和症状同样小。** 最小化修复的意思是"用最少的改动消除这个症状"——不顺手重构、不重命名、不"既然来了就把旁边的 `unwrap` 也换掉"。**顺手改动会让 D108 的回归验证失效**：diff 里混着几十行无关变化，你分不清哪一行修好了 bug、哪一行引入了新问题。

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 最小复现：一个文件名，不碰磁盘、不发网络，失败即指向解析函数
    #[test]
    fn absolute_episode_is_not_a_season() {
        let f = AnimeFile::parse("Bleach 366.mkv").unwrap();
        assert_eq!((f.season, f.episode), (1, 366)); // 修之前是 (2, 1) 或 (1, 0)
    }
}

// 修复：只在确实没有 SxxEyy 标记时才用绝对集号，并把 0 这个边界就地收口
fn season_episode_from_absolute(n: u32) -> (u32, u32) {
    if n == 0 { (1, 1) } else { (1, n) } // 别让 0 继续沿着后续计算往下走
}
```

⚠️ 三条：第一，修完只在真实目录上干跑看一眼就提交——**没有测试就等于没有证据**，下次改动会悄悄把它改回去；第二，测试里 `unwrap` 拿到 `None` 时报 **called Option::unwrap() on a None value**，这其实是好消息：说明**解析阶段**就没认出这个文件名，问题定位在 `parser.rs` 而不是 `plan.rs`；第三，**别把两个问题塞进一次修复**：集号偏移与剧场版误判是两件事，混着改，其中一个出了新问题就只能整体回退。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 报错在哪一行，bug 就在哪一行 | 报错点是症状；根因常在数据被算错的上一步 |
| 改完看着对就算修好 | 没有断言就没有证据；先把现象写成一条失败的测试 |
| 顺手把旁边的代码也清理掉 | 无关改动污染 diff，让回归验证失效 |
| 复现一定要真实目录 | 最小复现只要一个字符串；真实目录只用来最后确认 |

**在 tmdb-organizer 里**：P1 的根因落在 `parser.rs` 的 `AnimeFile::parse`（季/集拆分与 `EpisodeOrder` 的取值分支），`plan.rs` 的 `status_of` 只是把错的数据如实比出来——**从 `plan` 的症状倒推回 `parser` 的根因**是这两天的主线动作，修完就把上面那条用例并进 D104 那组测试里。

### D108 · 回归验证与提交规范

📖 [约定式提交 1.0.0（中文版）](https://www.conventionalcommits.org/zh-hans/v1.0.0/)

**回归验证是"修好了"加"没弄坏别的"。** 只跑新用例不够，副作用往往出现在没人看的输出里。三步：**新用例先失败**（证明它真能测到这个 bug，否则只是装饰）→ 改代码让它通过 → **全量 `cargo test` 后对同一份副本重跑干跑**，和 D106 存下的 `run-01.txt` 逐行对比。**差异恰好是目标那一行**才算通过。

**一个 commit 只装一个动机。** 原因很实际：`git revert` 只能整条撤销。如果一次提交里既有集号修复又有剧场版修复，事后发现集号那处改错了，你没法只撤一半。**首行说"做了什么"，正文说"为什么"**——首行是 `git log --oneline` 里唯一可见的部分，正文才是半年后（`git blame` 找到你时）真正需要的上下文：

| 写法 | 问题 |
| ---- | ---- |
| 更新一下 / fix bug / 修改 | 半年后无法判断动了什么，也没法据此生成发布说明 |
| fix(parser): 绝对集号 366 被误判为 S02E01 | 一眼看出动了哪个模块、症状是什么 |
| 首行之外补一段"为什么" | 记录取舍；不写，这个决定下次还会被人改回去 |

**提交规范不是形式主义，它是给未来的索引。** `fix:`、`feat:`、`docs:` 这类前缀让 `git log` 能按类型筛，也让工具能自动生成 changelog；"首行 50 字内"是为了在 `--oneline` 里不折行。**改完就提，不要攒**：攒成一大坨之后，无论是定位还是回滚，粒度都丢了。

```bash
# 1. 先让新用例失败：临时去掉修复再跑，确认它真的测到了这个 bug
cargo test absolute_episode    # 期望 failed；若直接通过，说明用例没测到东西

# 2. 恢复修复后，目标用例和全量测试都要跑
cargo test absolute_episode && cargo test

# 3. 对同一份副本重跑，和 D106 存下的基准逐行对比
cargo run --release -- ./bleach-test --dry-run > issues/run-02.txt 2> /dev/null
diff issues/run-01.txt issues/run-02.txt    # 差异应当只有目标那一行

# 4. 提交：一个动机一条提交；首行说"做了什么"，正文说"为什么"
git add src/parser.rs src/plan.rs
git commit -m "fix(parser): 绝对集号 366 被误判为 S02E01" \
           -m "文件名只有绝对集号时，parse 把 366 当成了季号；改为只在存在 SxxEyy 标记时拆季，并把 0 视为边界值。"
```

⚠️ 三条：第一，把全部说明塞进首行、不留正文，`git log --oneline` 里就是一行 80 字长句，**该说的"为什么"全部丢失**——第二条 `-m` 不是可选项；第二，diff 有输出不一定是坏事（要的正是"只差目标那一行"），但若出现**额外差异**，说明改到了不该改的地方，先回退再定位；第三，有改动却没 `git add` 就提交，得到 **nothing to commit, working tree clean**——先看 `git status`，别急着重跑。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 新用例通过就是修好 | 还要全量测试与干跑输出 diff；药会有副作用 |
| 提交信息是写给自己看的 | 是写给半年后的人（大概率是你）看的，重点是"为什么" |
| 一条提交里多修几个更省事 | 回退只能整条撤；一个动机一条提交才有粒度 |
| `cargo test` 绿了就万无一失 | 测试只覆盖已写下的用例；副本上的真实输出才是最终证据 |

**在 tmdb-organizer 里**：这次提交只动 `parser.rs`（外加并入 `plan.rs` 的测试模块里那一条新用例），并划掉 D106 清单里的 P1；`issues/run-01.txt` 与 `run-02.txt` 的 diff 就是"回归通过"的可查证据——`fix(parser):` 这样的前缀从今天起成为习惯。

### D109 · GitHub Actions 工作流（test / clippy / fmt）

📖 [GitHub Actions 文档](https://docs.github.com/zh/actions)、[dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain)

**为什么把这三条命令搬进 CI。** 本地跑不跑靠自觉，而**忙的时候人一定会跳过**。CI 把 D103 的 fmt/clippy 与 D104 的 `cargo test` 变成 push 的门槛：它不帮你写代码，只保证"合进去的一定是绿的"。价值不在"多一层检查"，而在**规则不再依赖自觉**——所以本地要先跑，CI 只是兜底。

**CI 的每一步都发生在一次"干净的克隆"里，这决定了工作流长什么样。** runner 上既没有你的 `.env`、也没有 `target/` 缓存，还可能没装 rustfmt/clippy 组件。所以顺序是：`checkout` 拉代码 → 装工具链（显式带上 `rustfmt`、`clippy`）→ 跑三条命令。**TMDB key 绝不能进仓库**：仓库里只提交 `.env.example`，真值放进仓库的 **Repository secret**，工作流里用 `secrets.TMDB_API_KEY` 注入。

**CI 里的命令和本地"看起来一样"，其实有两处不同。** 一是 `cargo fmt` 在本地会**改文件**，在 CI 里必须写 `cargo fmt --check`——它只报错、不改文件，否则格式问题会被静默放过。二是 clippy 默认只警告，要 `cargo clippy -- -D warnings` 才会让警告变成失败。另外，**本项目的测试其实不需要 key**：`parser`/`plan` 都是纯函数（D91），测试不碰网络；key 只有加真实 API 集成测试时才用得上。

```yaml
name: CI

on:
  push:
    branches: [main]        # 推 main 时跑
  pull_request:             # 提 PR 时也跑

env:
  CARGO_TERM_COLOR: always  # 日志带颜色，失败时更好读

jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4        # 干净克隆：这里没有 .env，也没有 target/
      - uses: dtolnay/rust-toolchain@stable
        with:
          components: rustfmt, clippy    # 这两个组件默认没装，必须显式带上
      - run: cargo fmt --check           # 只报不改：CI 里不能改文件
      - run: cargo clippy -- -D warnings # 警告升级为错误，有警告即失败
      - run: cargo test                  # 纯函数测试，不碰网络、不需要真实目录
        env:
          # 真值放仓库 secret，绝不要把 .env 提交上去
          TMDB_API_KEY: ${{ secrets.TMDB_API_KEY }}
```

⚠️ 三条：第一，`.env` 一旦被提交，**git 历史里永久留存**——GitHub 的 secret scanning 只是提醒，为时已晚；正确做法是 `git rm --cached .env` 并**换一把新 key**，旧 key 直接视为泄露；第二，忘了 `actions/checkout`，工作目录里没有源码，第一步就报 **error: could not find Cargo.toml in ... or any parent directory**，忘了带 `components` 则报 **error: no such command: clippy**；第三，CI 里写不带 `--check` 的 `cargo fmt` 不会失败——**它会"绿着放过"格式问题**，等于没检查。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| CI 是额外的检查项 | 它是这三条命令唯一的强制点；本地跑只是提前几分钟发现 |
| CI 里能用本地那份 `.env` | runner 是干净克隆；敏感值走 Repository secret |
| 放 `cargo fmt` 进去就够了 | 它只改文件、不报错；必须写 `cargo fmt --check` |
| 测试要连 TMDB 才能跑 | `parser`/`plan` 是纯函数，测试不碰网络，也不需要 key |

**在 tmdb-organizer 里**：工作流放在仓库的 `.github/workflows/ci.yml`（"最终文件结构"之外新增的一层目录），三步正好对上 D103 的 fmt/clippy 与 D104 的 `cargo test`；**push 之后那个绿勾，就是"别人 clone 下来能跑"的公开证据**。

### D110 · LICENSE 与 `Cargo.toml` 元数据

📖 [书 §14.2](https://kaisery.github.io/trpl-zh-cn/ch14-02-publishing-to-crates-io.html)

**为什么 Rust 生态默认"双许可"。** 惯例是 `MIT OR Apache-2.0`：**使用者可以任选其一**，MIT 简单，Apache-2.0 附带专利授权条款，两者合起来能覆盖绝大多数下游项目。**这不是格式问题，而是给下游的法律许可**——没有 LICENSE 文件与 `license` 字段时，别人"默认拿不到授权"，代码就在他手上也不能用。**别自己编许可**：`license` 是 SPDX 表达式，`OR` 必须大写、名字必须精确（`Apache-2.0` 不能写成 `Apache2`）；选双许可时习惯放两份文本（`LICENSE-MIT`、`LICENSE-APACHE`），跟字段里的表达式对应上。

**`Cargo.toml` 的 `[package]` 元数据是"给别人看的清单"。** 它会出现在 crates.io 页面，也是 `cargo package` 的校验对象。**包名要和 README「最终文件结构」里的目录名一致**：`name = "tmdb-organizer"`；二进制名默认与包名相同，所以用户拿到的是 `tmdb-organizer`（Windows 上是 `tmdb-organizer.exe`）——**名字不一致会让"安装"一节的命令全部失效**。

**`cargo package` 是发布前的排练。** 它按规则把该发布的文件打成一个 `.crate` 并做本地校验，**不碰任何外部状态**；`cargo publish` 才真的上传。两者分工：

| 命令 | 干什么 | 会改外部状态吗 |
| ---- | ---- | -------------- |
| `cargo package --list` | 只列出"将会被打包的文件"，检查有没有多带东西 | 不会 |
| `cargo package` | 本地打包 + 校验（含一次构建） | 不会 |
| `cargo publish` | 打包后上传到 crates.io，**版本号一经上传不可覆盖** | 会上传，不可撤 |

本项目是给用户装的应用、不是库，**不发布到 crates.io**；但跑一次 `cargo package` 依然值得——它能确认元数据合法，也能暴露"哪些文件被意外打进了包里"（实测：`.gitignore` 里忽略的 `.env` 不会被带进去）。

```toml
[package]
name = "tmdb-organizer"          # 与 README「最终文件结构」里的目录名一致
version = "1.0.0"                # 与 D111 的 tag v1.0.0 对应（tag 带 v，这里不带）
edition = "2021"                 # cargo new 生成的默认值；你的 cargo 较新时可能是 "2024"
description = "按 TMDB 集数顺序整理剧集文件"
license = "MIT OR Apache-2.0"    # SPDX 表达式，OR 必须大写；许可文本放 LICENSE 文件
repository = "https://github.com/你的用户名/tmdb-organizer"
readme = "README.md"
keywords = ["cli", "tmdb", "rename"]
categories = ["command-line-utilities"]

[dependencies]
# 与「依赖添加时间线」一致：clap / dotenvy / ureq / serde / serde_json / anyhow

[profile.release]
codegen-units = 1
lto = true
strip = true
panic = "abort"
```

⚠️ 三条：第一，元数据没补全时 `cargo package` **不会失败**，只给一行警告 **warning: manifest has no description, license, license-file, documentation, homepage or repository**——本地照样绿，但真到 `cargo publish` 那一步会被直接拒（发布必须有 description 与 license），**警告就是"还没准备好发布"的信号**；第二，`license-file` 指向的文件不存在时，`cargo package` 直接报 **error: license-file LICENSE-X does not appear to exist (relative to ...)**，包名写了空格也会停在 **error: invalid character in package name: ..., characters must be Unicode XID characters**——这两条是第一道关卡；第三，**别在 `Cargo.toml` 里留旧版本的残留**：改过 GPL 又改成 MIT 却忘了同步 LICENSE 文本，等于用一个字段承诺了不存在的东西。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 没有 LICENSE 也能被人用 | 默认没有授权；LICENSE 与 `license` 字段缺一不可 |
| 许可证随便抄一段就行 | 用 SPDX 表达式（MIT 或 MIT OR Apache-2.0），文本要与之一致 |
| `cargo package` 等于发布 | 它只在本地打包校验；上传是 `cargo publish` 的事 |
| 包名和目录名不一致没关系 | 二进制名跟着包名走，README 里的命令会全部失效 |

**在 tmdb-organizer 里**：`Cargo.toml` 的 `name = "tmdb-organizer"`、`license = "MIT OR Apache-2.0"` 配上仓库根目录的 LICENSE，与 `cargo package` 通过一起构成 D110 的验收；`[profile.release]` 那几行就是 D105 定下的优化配置，**没被这轮改动碰掉**。

### D111 · 打 tag 与 GitHub Release 流程

📖 [GitHub 文档：管理 Release](https://docs.github.com/zh/repositories/releasing-projects-on-github/managing-releases-in-a-repository)

**tag 和 Release 不是一回事。** tag 是 git 里指向某个 commit 的**标签**；Release 是 GitHub 在这个 tag 之上加的**一页说明 + 附件（二进制）**。**Release 必须挂在一个 tag 上**，所以顺序是固定的：先确认 main 是绿的（D109）→ 打 tag → 推 tag → 在 tag 上建 Release。**别在网页上一路点下去**：仓库里没有 tag 时，Release 页面只能让你现建一个，容易把版本号打在工作区的最新提交上。

**为什么要把二进制附在 Release 上。** 你的用户可能根本没装 Rust，`cargo build --release` 对他们不是"安装步骤"，而是"先去学一门语言"。D105 产出的 `target/release/tmdb-organizer` 直接作为附件上传，用户下载即用——**这正是 README"安装"一节里那条"或下载 Release"的兑现**。

**`v1.0.0` 里的 1 是一个承诺。** 语义化版本：**MAJOR**（参数删掉或改名、输出格式变了）、**MINOR**（向后兼容地加功能，比如新的 `--verbose` 级别）、**PATCH**（只修 bug）。**tag 名按惯例带 `v` 前缀，`Cargo.toml` 的 `version` 不带**——`v1.0.0` 与 `version = "1.0.0"` 必须对得上，否则用户 `--version` 打出来的版本和 Release 页面对不上，报 bug 时你无法判断他装的是哪一版。

```bash
# 0. 发布的前提：main 是绿的（D109 的 CI 过了），工作区干净
git status                    # 没有 .env、没有 target/ 之类的意外文件
git log --oneline -1          # 确认要发布的正是这个 commit

# 1. 版本号对齐：Cargo.toml 写 1.0.0，tag 写 v1.0.0
grep '^version' Cargo.toml    # version = "1.0.0"
git tag -a v1.0.0 -m "tmdb-organizer v1.0.0：首个可用版本"   # -a 建带说明的附注标签

# 2. 先本地构建出要附上的产物
cargo build --release         # target/release/tmdb-organizer（Windows 是 .exe）

# 3. 推 tag：git push 不会自动推 tag，必须显式推
git push origin v1.0.0

# 4. 建 Release：网页 Releases -> Draft a new release -> 选 tag v1.0.0，
#    写发布说明、附上二进制；装过 gh 也可以一条命令搞定：
#    gh release create v1.0.0 --notes "首个可用版本" ./target/release/tmdb-organizer
```

⚠️ 三条：第一，**`git push` 默认不推 tag**，只推分支——忘了 `git push origin v1.0.0`，GitHub 上根本没有这个 tag，Release 也就无从创建；第二，tag 打在了错的 commit 上：本地 `git tag -d v1.0.0` 可以删，**但已推送的 tag 还要在远端删**（`git push origin :v1.0.0`），而且**别人可能已经据此构建过**，所以要当事故处理；第三，`Cargo.toml` 还停在 `0.1.0` 而 tag 写成 `v1.0.0`，用户报 bug 时你看到的版本信息就是错的——**发布前先对齐这两处**。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| tag 和 Release 是一回事 | tag 是 git 标签；Release 是 GitHub 挂在 tag 上的说明与附件 |
| `git push` 会把 tag 一起推上去 | 不会，tag 要显式推：`git push origin v1.0.0` |
| 版本号随便起，反正自己用 | 1.0.0 是"接口稳定"的承诺；改参数名就该升 MAJOR |
| 发布后发现问题就覆盖同一个 tag | tag 不可变；重发要用新 tag（v1.0.1），别 force push 旧 tag |

**在 tmdb-organizer 里**：Release 的说明就是 README"安装/用法"两节的浓缩；附件用 D105 的 `target/release/tmdb-organizer`；tag `v1.0.0` 与 `Cargo.toml` 的 `version = "1.0.0"` 一一对应——**发布后把 Release 链接补进 README 顶部**。

### D112 · 项目复盘与产出总结

📖 [书 §14.3 Cargo 工作空间](https://kaisery.github.io/trpl-zh-cn/ch14-03-cargo-workspaces.html)（下一步要拆多个 crate 时从这看起）

**复盘不是感想，是回答三个可验证的问题。** ① 这个工具**现在能做什么**（不看代码，说得出来吗）？② 哪一周卡得最久、**为什么**卡在那？③ 同类问题下次出现，**第一步做什么**？写下来的价值在于：三周后你只会记得"当时挺难的"，具体判断全部消失。**这三问比"我学到了很多"有用得多**，因为最后一个答案能直接变成下次的动作。

**"完成"是五样东西，代码只是其中之一。** 能跑的二进制（D111 的 Release）、别人照着能走通的 README（D102）、在别人机器上也能复现的 CI 绿勾（D109）、LICENSE（D110）、一条读得懂的提交历史（D108）。**少任何一样，"别人 clone 下来能用"就不成立**——这也是前 15 周的技术内容与最后 8 天的流程内容合起来才叫一个项目的原因。

**"不再入门"的标准是能解释取舍，不是记住语法。** 入门阶段能写出跑得起来的东西；毕业的标准是**说清为什么这样写**：为什么 `display_label` 收 `&self` 而不是 `self`、为什么集数顺序用枚举而不是字符串、为什么 CI 里必须写 `cargo fmt --check`。README 顶部那句话能站住，靠的不是自信，而是**后面每一步都有证据**。

```bash
# 毕业自检：五条命令全过，才算"完成"，而不只是"写完了"
cargo fmt --check                                  # 1. 风格一致
cargo clippy -- -D warnings                        # 2. 没有绕弯路的写法
cargo test                                         # 3. 边界用例全绿
cargo build --release                              # 4. 产物能构建：target/release/tmdb-organizer
ls -1 LICENSE README.md .github/workflows/ci.yml   # 5. 许可证、文档、CI 都在仓库里

# 最后一步不在命令行里：在 README 顶部写下这句话，并挂上 Release 链接
# 我不再入门 Rust。我用 Rust 写了 tmdb-organizer。
```

⚠️ 三条：第一，"学完 16 周"不等于"技术栈都懂了"——`async`、`unsafe`、多线程都还是空白，**但你已经有能力自己查文档、写最小复现、让 CI 兜底**；第二，复盘写成流水账（"第 3 周学了所有权"）没有价值，要写"踩过的坑 + 现在的判断"，每条都能落成一个动作；第三，项目一停，README 顶部那句话就成了无凭无据的口号——**让它和 Release 链接、CI 绿勾放在一起**，半年后的你才验证得了。

**常见误解 vs 实际行为**

| 误解 | 实际 |
| ---- | ---- |
| 学完 16 周就等于会 Rust | 会的是"能独立做出工具"；async / unsafe / 并发仍是空白 |
| 复盘就是写感受 | 复盘要写成下次能照做的动作，最好带一个具体例子 |
| 项目"能跑"就算完成 | 完成 = 二进制 + README + CI + LICENSE + 提交历史 |
| 毕业那句话是给别人看的 | 它是为自己定的验收标准：拿证据说话 |

**在 tmdb-organizer 里**：README 顶部那句"我不再入门 Rust。我用 Rust 写了 tmdb-organizer。"与 D111 的 Release 页面互相引用；`issues/run-01.txt`、`run-02.txt` 加上这两天的 `fix(parser):` 提交，就是 16 周留下的可查痕迹——**从 D1 的 `cargo new tmdb-organizer` 到今天的 `v1.0.0`，这条路走完了**。

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
