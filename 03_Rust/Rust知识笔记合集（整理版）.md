# Rust 知识笔记合集（整理版）

> 本文为「编程知识库」Rust 方向的第一份整理文档，参照官方《The Rust Programming Language》（Rust 官方书，`doc.rust-lang.org/book`，源码仓库 `rust-lang/book`）、Rust 中文文档（`rustwiki.org/zh-CN/book`）与 Rust Reference / Rust by Example 的知识体系整理而成。
> 面向有 Java / Go 基础的后端开发者，按「语法 → 所有权 → 类型系统 → 并发 → 工程化」组织。文中对易错点做了 **【注意】** 标注，补充了常见面试/实战考点 **【补充】**。
> 整理日期：2026-11-01（涉及 Edition 2015 / 2018 / 2021 / 2024）。

## 目录

1. Rust 简介与工具链
2. 基础语法（变量、类型、函数、控制流）
3. 所有权（Ownership）—— Rust 的核心
4. 借用与引用（Borrowing）
5. 生命周期（Lifetimes）
6. 结构体、枚举与模式匹配
7. 常用集合（Vec / String / HashMap）
8. 错误处理（panic / Result / ?）
9. 泛型、Trait 与 Trait 对象
10. 闭包与迭代器
11. 智能指针（Box / Rc / Arc / RefCell / Deref / Drop）
12. 并发编程（线程 / 消息传递 / 共享状态 / Send + Sync）
13. 异步编程（Future / async-await / Tokio）
14. 模块、Crate 与 Cargo
15. Unsafe、宏与高级特性
16. 与 Java / Go 的对比
17. 高频面试题

---

## 一、Rust 简介与工具链

Rust 是一门注重 **内存安全、并发安全、零成本抽象** 的系统级编程语言。它不依赖 GC，而是通过 **所有权 + 借用检查（borrow checker）** 在编译期保证内存与线程安全。

### 1.1 语言特点

- **无 GC 的内存安全**：编译期检查所有权与借用，无悬垂指针、无数据竞争、无空指针（用 `Option` 代替 `null`）。
- **零成本抽象**：泛型、迭代器、Trait 等高层抽象在编译后与手写底层代码性能相当（单态化）。
- **并发安全**：`Send` / `Sync` 编译期约束，把「数据竞争」变成编译错误（"Fearless Concurrency"）。
- **强类型 + 代数数据类型**：`enum` 可携带数据，配合 `match` 穷尽匹配。
- **实用工具链**：`cargo`（构建/依赖/测试/发布）、`rustfmt`（格式化）、`clippy`（lint）、`rustup`（工具链管理）。

### 1.2 安装与版本

- 通过 `rustup` 安装：`rustup toolchain install stable`，`rustup update`。
- 查看版本：`rustc --version`、`cargo --version`。
- **Edition（版次）**：2015 / 2018 / 2021 / 2024，Edition 是语法档位而非编译器版本，不同 crate 可用不同 Edition 混编。

### 1.3 Cargo 常用命令

| 命令 | 作用 |
|---|---|
| `cargo new hello` / `cargo new --lib mylib` | 新建二进制 / 库项目 |
| `cargo build [--release]` | 编译（debug / release） |
| `cargo run` | 编译并运行 |
| `cargo check` | 只做类型检查，速度比 build 快，开发时常用 |
| `cargo test` | 运行测试 |
| `cargo fmt` / `cargo clippy` | 格式化 / 静态检查 |
| `cargo add serde` | 添加依赖（写回 Cargo.toml） |

项目结构：`Cargo.toml`（包元数据与依赖）、`Cargo.lock`（锁定依赖版本，二进制项目应提交）、`src/main.rs`（二进制入口）、`src/lib.rs`（库入口）。

---

## 二、基础语法

### 2.1 变量与可变性

```rust
let x = 5;          // 不可变绑定（默认）
let mut y = 10;     // 可变绑定
y += 1;
let x = x + 1;      // 变量遮蔽（shadowing）：可改变类型，重新绑定新变量
```

**【注意】** `let` 默认不可变；`mut` 才能修改。遮蔽（shadowing）会创建新变量，与原变量可不同类型；而 `mut` 只是允许修改同一个变量。

### 2.2 数据类型

- 标量：整数（`i8/i16/i32/i64/i128`、`u8..u128`、`isize/usize`）、浮点（`f32/f64`）、布尔 `bool`、字符 `char`（4 字节，Unicode 标量值）。
- 复合：元组 `(T1, T2)`（可不同类型）、数组 `[T; N]`（同类型、长度固定）。
- **整数溢出**：debug 模式溢出会 panic；release 模式按二进制补码回绕（wrapping）。需要明确行为时用 `checked_/wrapping_/saturating_/overflowing_` 系列方法。

### 2.3 函数与控制流

```rust
fn add(a: i32, b: i32) -> i32 { a + b }   // 最后表达式即返回值（无分号）

let n = 3;
if n > 0 { println!("positive"); }        // if 是表达式，可赋值
let v = if n > 0 { 1 } else { -1 };
```

- `if` / `else`、`loop`、`while`、`for ... in ...`（`for` 是遍历迭代器最常用姿势）。
- `loop` 可作为表达式，用 `break value;` 返回值。
- **【注意】** 表达式（expression）与语句（statement）的区别：最后一行不加分号即为返回值。

---

## 三、所有权（Ownership）—— Rust 的核心

### 3.1 三条所有权规则

1. Rust 中的每一个值都有一个 **所有者（owner）**。
2. 同一时刻，值 **只有一个所有者**。
3. 当所有者 **离开作用域** 时，值会被自动 `drop`（释放）。

### 3.2 移动（Move）与克隆（Clone）

```rust
let s1 = String::from("hello");
let s2 = s1;            // 所有权从 s1 移动到 s2；此后 s1 不可用
// println!("{s1}");    // 编译错误：borrow of moved value

let s3 = s2.clone();    // 深拷贝，堆数据复制一份
```

**【注意】** 实现了 `Copy` 的类型（如整数、浮点、`bool`、`char`、只含 `Copy` 元素的元组/数组）在赋值时是「复制」而非「移动」，原变量仍可用。自定义类型实现 `Copy` 需要其所有字段都实现 `Copy`。

### 3.3 所有权与函数

把值传给函数与赋值语义相同：传入 `String` 会把所有权移动进函数；函数返回的 `String` 可把所有权移出。因此常配合「返回所有权」或「传引用」来避免反复移动。

**【补充】** 所有权机制带来的好处：无需 GC 也能确定性回收；编译期杜绝 use-after-free 与 double-free。

---

## 四、借用与引用（Borrowing）

引用（`&T` / `&mut T`）允许在不获取所有权的情况下访问数据，称为「借用」。

### 4.1 借用规则（核心）

- 在任意时刻，对同一数据，要么 **只有一个可变引用 `&mut T`**，要么 **任意多个不可变引用 `&T`**，二者不能同时存在。
- 引用必须始终有效（不能有悬垂引用）。

```rust
let mut s = String::from("hello");
let r1 = &s;      // 不可变借用
let r2 = &s;      // 可以有多个不可变借用
// let r3 = &mut s; // 编译错误：不能在存在不可变借用时创建可变借用
println!("{r1} {r2}");

let r4 = &mut s;  // r1/r2 最后一次使用后，NLL 允许此时创建可变借用
r4.push_str(" world");
```

### 4.2 切片（Slice）

切片是对集合中连续一段的引用，不获取所有权：`&[T]`、`&str`。

```rust
let a = [1, 2, 3, 4, 5];
let slice: &[i32] = &a[1..3];   // 指向 a[1], a[2]

let s = String::from("hello world");
let hello: &str = &s[0..5];      // 字符串切片
```

**【补充】NLL（Non-Lexical Lifetimes）**：自 Edition 2018 起，借用的生命周期以「最后一次使用」为准而非作用域末尾，极大减少了不必要的借用冲突。

---

## 五、生命周期（Lifetimes）

生命周期注解用来告诉编译器多个引用之间的存活关系，**不会延长**实际寿命，只为借用检查提供依据。常见语法：`&'a T`、`<'a>`。

### 5.1 需要标注的典型场景

```rust
// 返回值的生命周期取决于两个入参中的一个，编译器无法自动推断
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() { x } else { y }
}
```

### 5.2 生命周期省略三规则（Elision）

编译器在无显式注解时按以下规则推断：

1. 每个输入引用各自分配一个生命周期参数。
2. 若只有一个输入生命周期，则它被赋给所有输出生命周期。
3. 若有多个输入生命周期，但其中一个是 `&self` 或 `&mut self`，则 `self` 的生命周期赋给所有输出。

若三条规则后仍无法确定，编译器报错，需手动标注。

### 5.3 `'static`

`'static` 表示引用可存活整个程序运行期（如字符串字面量 `"..."` 的类型是 `&'static str`）。**【注意】** 不要滥用 `'static` 去「解决」生命周期错误，应优先理清真实的所有权关系。

**【补充】** 结构体若持有引用，需要生命周期参数：`struct Parser<'a> { input: &'a str }`。

---

## 六、结构体、枚举与模式匹配

### 6.1 结构体

```rust
struct User { name: String, age: u32 }
impl User {
    fn new(name: &str, age: u32) -> Self {
        Self { name: name.to_string(), age }
    }
    fn is_adult(&self) -> bool { self.age >= 18 }
}
let u2 = User { age: 20, ..User::new("a", 18) };  // 结构体更新语法：剩余字段取自另一实例
```

### 6.2 枚举与 `Option`

```rust
enum Msg { Quit, Move { x: i32, y: i32 }, Write(String) }

// Option 取代 null：Some(v) / None
let x: Option<i32> = Some(5);
```

**【补充】** `Option<T>` 从类型层面消除了空指针；`?` 运算符与 `match`/`if let` 配合使用可优雅地处理 `Option` 与 `Result`。

### 6.3 模式匹配

```rust
match x {
    Some(n) if n > 0 => println!("positive {n}"),  // 匹配守卫
    Some(0) => println!("zero"),
    Some(n @ 1..=9) => println!("digit {n}"),      // @ 绑定
    Some(_) | None => println!("other"),
}

if let Some(n) = x { /* ... */ }
while let Some(v) = stack.pop() { /* ... */ }
let Some(v) = opt else { return; };   // let...else（Edition 2021+）
```

**【注意】** `match` 必须 **穷尽** 所有分支；`_` 是通配符。`if let` / `while let` / `let...else` 是简化写法。

---

## 七、常用集合

### 7.1 Vec<T>

```rust
let mut v: Vec<i32> = Vec::new();
v.push(1); v.push(2);
let third = v.get(2);          // 返回 Option<&T>，越界返回 None（安全）
// v[100]                      // 索引越界会 panic
for i in &v { println!("{i}"); }
```

### 7.2 String 与 &str

- `String`：可增长、拥有所有权的 UTF-8 字符串（堆上）。`&str`：字符串切片（ borrowed）。
- **【注意】** `String` **不能用整数下标索引**（UTF-8 变长）；应按 `bytes()` / `chars()` 遍历，或切片（须落在合法字符边界，否则 panic）。
- 拼接：`push_str`、`+`（会移动左值）、`format!`（不夺所有权，最常用）。

```rust
let s = format!("{}-{}", "a", 1);
```

### 7.3 HashMap<K, V>

```rust
use std::collections::HashMap;
let mut m = HashMap::new();
m.insert("a", 1);
m.entry("a").or_insert(0);   // entry API：不存在则插入
*m.entry("b").or_insert(0) += 1;
```

**【注意】** key 需实现 `Eq + Hash`；`HashMap` 默认使用随机哈希种子（防 HashDoS）。

---

## 八、错误处理

### 8.1 不可恢复错误：panic!

- `panic!` 会展开（unwind）栈并终止当前线程；可配置 `panic = "abort"` 直接中止。
- 适用于「程序处于不可继续的非法状态」。

### 8.2 可恢复错误：Result<T, E>

```rust
enum Result<T, E> { Ok(T), Err(E) }

fn read_num(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>()
}

fn run() -> Result<(), Box<dyn std::error::Error>> {
    let n = read_num("42")?;   // ? 出错则提前返回 Err（会做 From 转换）
    println!("{n}");
    Ok(())
}
```

- `?` 运算符：出错时把错误向上传播，并用 `From` 做错误类型转换。
- 组合子：`map`、`and_then`、`unwrap_or`、`unwrap_or_else`、`ok_or`。
- 生产代码常用 `thiserror`（定义错误枚举）与 `anyhow`（应用层错误上下文）。

**【补充】选型**：库代码返回具体错误类型（`thiserror`），应用/`main` 用 `anyhow::Result` 方便传播与附加上下文。避免在库中用 `unwrap()/expect()` 处理可预期错误。

---

## 九、泛型、Trait 与 Trait 对象

### 9.1 泛型

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut max = &list[0];
    for item in &list[1..] { if item > max { max = item; } }
    max
}
```

**【补充】** Rust 泛型是 **单态化（monomorphization）**：编译期为每个具体类型生成专用代码，运行时零开销（对比 Java 泛型的类型擦除）。

### 9.2 Trait（类似接口）

```rust
trait Summary {
    fn summarize(&self) -> String;
    fn preview(&self) -> String {           // 默认方法
        format!("Read more: {}", self.summarize())
    }
}
impl Summary for Article {
    fn summarize(&self) -> String { self.title.clone() }
}
```

- Trait 约束：`fn f<T: Summary + Clone>(x: T)`，或用 `where` 子句。
- `impl Trait`：参数位置 `fn f(x: impl Summary)`；返回位置 `fn g() -> impl Summary`。

### 9.3 静态分发 vs 动态分发（Trait 对象）

- **泛型 / `impl Trait`**：静态分发，编译期单态化，无运行时开销。
- **`dyn Trait`（Trait 对象）**：动态分发，通过虚表（vtable）在运行时调用；写法 `&dyn Trait` / `Box<dyn Trait>`，适合「同一集合存放多种实现」。

**【注意】** Trait 对象要求 Trait 满足 **对象安全（object safety）**（如方法不能返回 `Self`、不能有泛型方法等）。

---

## 十、闭包与迭代器

### 10.1 闭包

```rust
let add = |a, b| a + b;
let mut count = 0;
let mut inc = || count += 1;   // 捕获 &mut count（FnMut）
inc();
let take = move || count;      // move 捕获（拥有）
```

三种捕获 trait：`Fn`（不可变借用）、`FnMut`（可变借用）、`FnOnce`（消耗/移动捕获）。

### 10.2 迭代器

```rust
let v = vec![1, 2, 3, 4];
let sum: i32 = v.iter().filter(|&&x| x % 2 == 0).map(|&x| x * 2).sum();
let got: Vec<i32> = v.iter().copied().take(2).collect();
```

- 迭代器是 **惰性的**：适配器（`map`、`filter`、`take`、`zip`、`enumerate`）不触发求值，消费器（`collect`、`sum`、`fold`、`count`、`for_each`）才驱动。
- **【补充】** 迭代器链通常比手写 `for` 循环更快或相当（编译器优化充分），且更安全清晰。

---

## 十一、智能指针

智能指针是拥有额外元数据/能力、实现 `Deref`（或 `DerefMut`）与 `Drop` 的类型。

| 类型 | 所有权/线程 | 用途 |
|---|---|---|
| `Box<T>` | 独占、堆分配 | 递归类型、`Box<dyn Trait>`、转移大数据 |
| `Rc<T>` | 共享、**非线程安全** | 单线程内多所有者（引用计数，`clone` 增计数） |
| `Arc<T>` | 共享、**线程安全**（原子计数） | 多线程共享只读数据 |
| `RefCell<T>` | 独占、**运行时借用检查** | 内部可变性（单线程） |
| `Cell<T>` | 独占、Copy 值 | 内部可变性（轻量，无借用引用） |
| `Mutex<T>` / `RwLock<T>` | 共享、线程同步 | 多线程可变共享（配 `Arc`） |

```rust
use std::rc::Rc;
use std::cell::RefCell;
let shared = Rc::new(RefCell::new(vec![1, 2, 3]));
shared.borrow_mut().push(4);   // 运行时借用检查
```

**【注意】**
- `Rc` / `RefCell` 不能跨线程（`Rc` 非 `Send`，`RefCell` 非 `Sync`）。
- `RefCell` 的借用规则违规在 **运行时** panic（而 `&`/`&mut` 是编译期检查）。
- 用 `Weak<T>` 打破 `Rc` 引用环，避免内存泄漏。
- 内部可变性 = 通过 `&self` 修改内部数据（`RefCell` / `Cell` / `Mutex` / `RwLock` / `Atomic*`）。

---

## 十二、并发编程

### 12.1 线程

```rust
use std::thread;
let h = thread::spawn(move || {    // move 把所有权移入线程
    println!("hi from thread");
});
h.join().unwrap();                 // 等待线程结束
```

### 12.2 消息传递（channel）

```rust
use std::sync::mpsc;
let (tx, rx) = mpsc::channel();
thread::spawn(move || { tx.send(42).unwrap(); });
println!("{}", rx.recv().unwrap());
```

`mpsc` = 多生产者单消费者；`sync_channel` 为有界通道。

### 12.3 共享状态

```rust
use std::sync::{Arc, Mutex};
let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];
for _ in 0..10 {
    let c = Arc::clone(&counter);
    handles.push(thread::spawn(move || {
        let mut n = c.lock().unwrap();
        *n += 1;
    }));
}
for h in handles { h.join().unwrap(); }
```

**【注意】** `Mutex` 的守卫（`MutexGuard`）在离开作用域时自动解锁；注意避免「持锁跨 `.await`」与死锁。

### 12.4 Send 与 Sync（编译期并发安全）

- **`Send`**：该类型的 **所有权** 可在多个线程间转移（几乎大多数类型是 `Send`，`Rc<T>` 不是）。
- **`Sync`**：该类型的 **引用 `&T`** 可安全地在多个线程间共享（即 `&T: Send`）。
- 二者是标记 trait（无方法），由编译器自动推导；组合类型若所有字段都 `Send`/`Sync` 则自动实现。

**【补充】** 这就是 Rust「无畏并发」的底层：数据竞争被 `Send/Sync` 约束在编译期拦住。

---

## 十三、异步编程

### 13.1 Future 与 async/await

```rust
async fn fetch() -> String { "data".to_string() }

async fn run() {
    let a = fetch().await;   // .await 挂起直到 Future 完成
    println!("{a}");
}
```

- `async fn` 返回一个实现 `Future` 的匿名类型；`Future` 是惰性的，**必须被 executor 轮询（poll）** 才会推进。
- `.await` 让出执行权（不阻塞线程），适合作 I/O 密集场景。
- **Rust 标准库不含运行时**，需引入执行器：最主流是 **Tokio**（另有 async-std、smol）。

### 13.2 关键概念

- `Pin<P>`：保证被指向对象地址固定（自引用 Future 需要）。
- `Stream`：异步版本的迭代器。
- `select!` / `join!`：并发组合多个 Future。

### 13.3 线程 vs 异步（选型）

| 维度 | 多线程 | 异步（async/await） |
|---|---|---|
| 适用 | CPU 密集 / 少量并发 | 高并发 I/O（网络/磁盘） |
| 开销 | 每线程栈较大，上下文切换成本 | 单线程可跑成千上万任务，切换轻量 |
| 生态 | `std::thread`、rayon | Tokio、async-std |
| 复杂度 | 相对简单 | 需理解 Future/Pin/运行时 |

**【注意】** 不要在 async 任务里做长时间阻塞操作（会堵住 executor 线程），应使用 `tokio::task::spawn_blocking`。

---

## 十四、模块、Crate 与 Cargo

### 14.1 模块系统

```rust
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}
use crate::front_of_house::hosting;   // crate / super / self 路径
```

- `pub` 控制可见性（默认私有）；`use` 引入路径到作用域；`mod` 声明模块（可拆到独立文件）。
- 一个 **package** 含一个或多个 **crate**；crate 是编译单元（`lib` / `bin`）。

### 14.2 Cargo 进阶

- **workspace**：多 crate 共享 `Cargo.lock` 与 target 目录，`[workspace] members = [...]`。
- **features**：条件编译开关，`[features] default = ["std"]`，用 `#[cfg(feature = "x")]`。
- **发布**：`cargo publish`（发布到 crates.io，需唯一名与版本，遵循 SemVer）。

---

## 十五、Unsafe、宏与高级特性

### 15.1 Unsafe Rust

`unsafe` 块获得五种「超能力」：解引用裸指针、调用 `unsafe` 函数、访问/修改可变静态变量、实现 `unsafe trait`、访问 `union` 字段。

**【注意】** `unsafe` **不会关闭借用检查器**，只是允许你承担额外的安全责任；应尽量封装在安全抽象内部并加文档说明不变量。

### 15.2 宏

- **声明式宏** `macro_rules!`：模式匹配式代码生成（如 `vec!`）。
- **过程宏（proc macro）**：`#[derive(...)]`（派生）、属性宏、函数式宏，需单独的 `proc-macro` crate。

### 15.3 高级类型

- **Newtype 模式**：`struct Meters(u32)` 做类型安全封装。
- **类型别名** `type Km = u32;`（不产生新类型）。
- **Never 类型 `!`**：表示永不返回（`panic!`、`loop {}`）。
- **DST 与 `Sized`**：动态大小类型（`str`、`[T]`、`dyn Trait`），通常通过引用/智能指针使用。
- **`PhantomData<T>`**：在类型中标记「逻辑上拥有 T」但不存储数据。

---

## 十六、与 Java / Go 的对比

| 维度 | Rust | Java | Go |
|---|---|---|---|
| 内存管理 | 所有权 + 借用，编译期，无 GC | GC | GC |
| 空值 | `Option<T>`，无 null | `null`（易 NPE） | `nil` |
| 错误处理 | `Result<T,E>` + `?` | 异常 try/catch | 多返回值 `(v, err)` |
| 泛型 | 单态化（零成本） | 类型擦除 | 泛型（较新，1.18+） |
| 并发 | 线程 + async，`Send/Sync` 编译期检查 | 线程 + 线程池 + 虚拟线程 | goroutine + channel |
| 接口/抽象 | Trait（可默认实现、关联类型） | interface | interface（隐式实现） |
| 编译产物 | 原生二进制 | JVM 字节码 | 原生二进制 |
| 学习曲线 | 陡（所有权/生命周期） | 中 | 平缓 |

---

## 十七、高频面试题

**1. Rust 的所有权解决了什么问题？**
在无 GC 前提下于编译期保证内存安全：单一所有者 + 离开作用域自动释放，杜绝 use-after-free、double-free、数据竞争。

**2. `String` 与 `&str` 的区别？**
`String` 拥有所有权、可增长（堆）；`&str` 是借用的字符串切片。函数参数优先用 `&str`（更通用），需要持有/修改时用 `String`。

**3. 为什么 `String` 不能用 `s[0]` 索引？**
字符串是 UTF-8 变长编码，按字节下标可能落在字符中间；应使用 `chars()` / `bytes()` 或切片（须在字符边界）。

**4. 移动（move）与克隆（clone）、Copy 的区别？**
move 转移所有权（源失效）；`clone` 深拷贝堆数据；实现 `Copy` 的类型按位复制且源仍有效（仅限无堆所有权的简单类型）。

**5. 借用规则是什么？**
同一时刻要么一个 `&mut T`，要么任意多个 `&T`；引用必须始终有效。

**6. 生命周期注解会延长数据寿命吗？**
不会。它只是描述引用间的存活关系，供借用检查器验证。

**7. Trait 与泛型：静态分发 vs 动态分发？**
泛型/`impl Trait` 是静态分发（单态化、零开销）；`dyn Trait` 是动态分发（虚表），适合异构集合。

**8. `Box` / `Rc` / `Arc` 的区别？**
`Box` 独占堆；`Rc` 单线程共享引用计数；`Arc` 原子引用计数、可跨线程。

**9. `RefCell` 与 `Mutex` 的区别？**
都提供内部可变性；`RefCell` 在运行时检查借用、单线程；`Mutex` 用锁做线程同步、可跨线程。

**10. `Send` 与 `Sync` 是什么？**
标记 trait：`Send` 表示所有权可跨线程转移；`Sync` 表示 `&T` 可跨线程共享。Rust 借此在编译期阻止数据竞争。

**11. `panic!` 与 `Result` 如何取舍？**
不可恢复的非法状态用 `panic!`；可预期的错误用 `Result` 传播（库代码避免 `unwrap`）。

**12. 什么情况用 `unsafe`？**
需要突破编译器保证（裸指针、FFI、底层数据结构）时；`unsafe` 不关闭借用检查，需自行保证不变量。

**13. 线程与 async 如何选型？**
CPU 密集/少量并发用线程；高并发 I/O 用 async（Tokio），注意不要阻塞 executor。

**14. 什么是单态化（monomorphization）？**
编译期为泛型的每个具体类型生成专用代码，带来零成本抽象，代价是编译时间与二进制体积增加。

**15. 如何避免 `Rc` 循环引用导致的内存泄漏？**
对「反向/弱引用」一端使用 `Weak<T>`，打破强引用环。

---

## 附录：学习资源

- 官方书《The Rust Programming Language》：https://doc.rust-lang.org/book/
- 中文版：https://rustwiki.org/zh-CN/book/
- Rust by Example：https://doc.rust-lang.org/rust-by-example/
- Rust Reference（语言规范）：https://doc.rust-lang.org/reference/
- Rust 标准库文档：https://doc.rust-lang.org/std/
- 异步编程《Asynchronous Programming in Rust》：https://rust-lang.github.io/async-book/
- Tokio 文档：https://tokio.rs/

---

_整理自 Rust 官方文档体系，仅供个人学习使用。_
