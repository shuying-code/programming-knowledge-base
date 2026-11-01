# Java 知识笔记合集（整理版）

> 本文由 `F:\知识库\01_Java\思维导图笔记` 目录下 29 个思维导图（.xmind）与文章/试题文件整理合并而成。
> 整理过程中对原图中的笔误、口误和过时结论做了修正（标注 **【修正】**），并补充了常见面试/实战考点（标注 **【补充】**）。
> 说明：部分原图重复（如 JVM、java并发编程之美、计算机网络总结各有两份），本文已去重合并。
> 生成日期：2026-11-01

## 目录

1. Java 基础与集合
2. JVM（类加载 · 运行时数据区 · 执行引擎 · 垃圾回收）
3. Java 并发编程
4. Spring 与 Spring Boot
5. MySQL 与数据库
6. Redis
7. Netty 与 NIO
8. 计算机网络
9. Linux 命令与 Shell
10. 设计模式
11. 数据结构与算法
12. 面试重点与分布式专题
13. Go 面试相关（兼收）
14. 附录：本次整理的主要修正与补充

---

## 一、Java 基础与集合

### 1.1 集合框架体系

- **Collection**（单列集合）
  - **List**（有序、可重复）：`ArrayList`、`LinkedList`、`Vector`（线程安全，已过时，`Stack` 继承自 Vector）
  - **Set**（无序/不可重复）：`HashSet`（底层 HashMap）、`LinkedHashSet`（维护插入顺序，底层 LinkedHashMap）、`TreeSet`（红黑树，可排序）
  - **Queue/Deque**：`ArrayDeque`、`PriorityQueue`、阻塞队列见并发章节
- **Map**（键值对）：`HashMap`、`LinkedHashMap`、`TreeMap`、`Hashtable`（线程安全、已过时）、`ConcurrentHashMap`（并发安全）、`WeakHashMap`、`IdentityHashMap`

### 1.2 各实现类要点

| 集合 | 底层结构 | 有序 | 线程安全 | 说明 |
|---|---|---|---|---|
| ArrayList | 动态数组 | 是（插入序） | 否 | 随机访问快，增删中间元素慢，默认容量 10，扩容 1.5 倍 |
| LinkedList | 双向链表 | 是 | 否 | 增删快、随机访问慢，同时实现 Deque |
| Vector | 动态数组 | 是 | 是（synchronized） | 方法级同步，性能差，已不推荐 |
| HashSet | HashMap | 否 | 否 | 元素作为 key，value 为常量占位 |
| LinkedHashSet | LinkedHashMap | 是（插入序） | 否 | 额外维护双向链表 |
| TreeSet | TreeMap（红黑树） | 是（排序） | 否 | 支持自然排序/Comparator |
| HashMap | 数组+链表+红黑树 | 否 | 否 | 见下方详述 |
| LinkedHashMap | HashMap+双向链表 | 是（插入/访问序） | 否 | 可实现 LRU（accessOrder=true） |
| TreeMap | 红黑树 | 是（排序） | 否 | 按 key 排序 |
| Hashtable | 数组+链表 | 否 | 是 | 全部加锁，key/value 不允许 null |
| ConcurrentHashMap | 数组+链表+红黑树 | 否 | 是 | 见并发章节 |

### 1.3 HashMap 重点【补充】

- 结构：JDK7 为「数组 + 链表（头插）」；**JDK8 为「数组 + 链表 + 红黑树」**。
- 关键参数：默认初始容量 **16**、负载因子 **0.75**、扩容阈值 = 容量 × 负载因子。
- 树化条件：链表长度 ≥ **8** 且数组容量 ≥ **64** 时转红黑树；元素减少到 ≤ **6** 时退化为链表。
- 扩容：容量翻倍，JDK8 利用位运算把节点拆成「低位链/高位链」，避免重新计算 hash。
- index 计算：`(n-1) & hash`；hash 经过扰动 `h ^ (h>>>16)` 降低碰撞。
- **JDK7 头插法在并发扩容时可能形成环形链表导致死循环；JDK8 改为尾插，但 HashMap 仍然线程不安全**（并发 put 仍可能丢数据/覆盖）。
- key 为何常用 String/Integer：不可变且已正确实现 `hashCode/equals`。

### 1.4 异常体系

- `Throwable`
  - `Error`：程序无法处理的严重错误（`OutOfMemoryError`、`StackOverflowError`），不应 catch。
  - `Exception`
    - 受检异常（Checked）：`IOException`、`SQLException` 等，编译期强制处理。
    - 运行时异常（Unchecked，RuntimeException）：`NullPointerException`、`IndexOutOfBoundsException`、`ClassCastException` 等。
- 【补充】`finally` 一定执行（除非 `System.exit`/JVM 崩溃）；`try-with-resources` 可自动关闭 `AutoCloseable`；`final`/`finally`/`finalize` 三者不同。

### 1.5 字符串

- `String` 不可变（final 字符数组/字节数组），`+` 拼接在循环中会不断产生新对象。
- **循环内拼接用 `StringBuilder`（非线程安全、更快）**；多线程用 `StringBuffer`（synchronized）。
- 【补充】字符串常量池在 **JDK7 起移入堆**；`intern()` 会把字符串放入常量池并返回引用。

### 1.6 equals 与 hashCode / fail-fast【补充】

- 约定：重写 `equals` 必须重写 `hashCode`，保证「相等的对象 hash 值一定相等」；否则放入 HashMap/HashSet 会行为异常。
- `==` 比较基本类型的值、或对象引用地址；`equals` 默认比较引用，通常被重写为比较内容。
- **fail-fast（快速失败）**：`ArrayList/HashMap` 等集合的迭代器在遍历时若集合被结构性修改（modCount 变化），抛 `ConcurrentModificationException`；遍历删除应使用 `Iterator.remove()` 或并发集合。
- `Comparable`（自然排序 compareTo）与 `Comparator`（外部比较器）区别。

---

## 二、JVM

### 2.1 类加载子系统

- **Class 文件结构**：魔数、版本号、常量池、访问标志、字段表、方法表、属性表（如 `Code` 属性存放方法字节码）。
- 高版本 JDK 能向下兼容旧版 Class，但不能运行更高版本的 Class。
- JVM 采用**面向操作数栈**（而非寄存器）的字节码架构；寄存器架构指令更紧凑，但需更多入栈/出栈与指令分派。

#### 类加载过程（生命周期七阶段）

加载 → 验证 → 准备 → 解析 → 初始化 → 使用 → 卸载。

- **加载**：通过全限定名获取二进制字节流；转为方法区运行时结构；在堆中生成 `java.lang.Class` 对象。
- **验证**：文件格式验证、元数据验证、字节码验证、符号引用验证。
- **准备**：为**静态变量**分配内存并赋**零值**（`static final` 常量在此阶段直接赋字面值）。
- **解析**：将常量池符号引用替换为直接引用（可在初始化之后进行，以支持动态绑定）。
- **初始化**：执行类构造器 `<clinit>()`，即静态变量赋值与静态代码块。
- 触发初始化的时机（主动引用）：`new`、访问/设置静态字段（`getstatic/putstatic`，被 final 修饰的编译期常量除外）、调用静态方法（`invokestatic`）、反射、初始化子类（先初始化父类）、作为启动类（main 所在类）。
- **被动引用不触发初始化**：通过子类访问父类静态字段、数组定义 `new A[10]`、访问编译期常量。

#### 类加载器与双亲委派

- 层次：`BootstrapClassLoader`（启动，C++ 实现，加载 `JAVA_HOME/lib`）→ `ExtensionClassLoader`（JDK9 后为 Platform）→ `ApplicationClassLoader`（应用类）→ 自定义加载器。
- **双亲委派**：收到加载请求先委派给父加载器，父加载器无法完成时才自己加载。
  - 好处：保证核心类库安全（避免被篡改）、避免类重复加载。
  - 【补充】打破双亲委派的场景：SPI（Thread Context ClassLoader）、Tomcat（WebAppClassLoader 隔离）、OSGi。JDK9 后为「模块化 + 委派」结构。

### 2.2 运行时数据区

| 区域 | 线程 | 存储内容 | 异常 |
|---|---|---|---|
| 方法区 / 元空间 | 共享 | 类信息、常量、静态变量、JIT 编译代码、运行时常量池 | OOM: Metaspace |
| 堆 | 共享 | 对象实例、数组（GC 堆） | OOM: Java heap space |
| 虚拟机栈 | 隔离 | 栈帧（局部变量表、操作数栈、动态链接、方法返回地址） | StackOverflowError / OOM |
| 本地方法栈 | 隔离 | Native 方法服务 | StackOverflowError / OOM |
| 程序计数器 | 隔离 | 当前线程执行字节码的行号指示器 | 唯一不会 OOM |
| 直接内存 | — | NIO `DirectByteBuffer` 分配的堆外内存 | OOM: Direct buffer memory |

- **方法区演进【修正】**：JDK7 及以前用「永久代」实现（在堆内）；**JDK7 已把字符串常量池移入堆**；**JDK8 移除永久代，改用「元空间 Metaspace」（位于本地内存）**。原文将「静态变量与字符串常量池移入堆」都归到 JDK8，实为 JDK7 与 JDK8 两次变化。
- **堆分代**：新生代（Eden + Survivor0 + Survivor1，默认 8:1:1）触发 Minor GC；老年代（Old）触发 Major/Full GC。JDK8 起无永久代；G1/ZGC 逻辑上分代、物理上按 Region 划分。
- **栈帧**：局部变量表、操作数栈、动态链接、方法返回地址。递归过深 → StackOverflowError。
- **运行时常量池**：方法区一部分；`String.intern()` 可在运行期向池中放入常量。
- 【补充】`-Xms/-Xmx` 堆初始/最大；`-Xmn` 新生代；`-Xss` 栈大小；`-XX:MetaspaceSize/-XX:MaxMetaspaceSize` 元空间；`-XX:+PrintGCDetails` 打印 GC 日志。

### 2.3 执行引擎

- **解释器**：逐条解释字节码，启动快、执行相对慢。
- **JIT 即时编译器（C1/C2）**：对「热点代码」编译为本地机器码。
  - 热点探测：采样式（可能受线程阻塞干扰）与**计数器式**（方法调用计数器 + 回边计数器）——HotSpot 采用计数器式。
  - C1 编译快、优化少；C2 编译慢、优化激进；分层编译（Tiered）综合两者。
- **Code Cache**：存放 JIT 编译后的机器码；占满后 JIT 停止编译、退化为解释执行，导致性能下降（可调 `-XX:ReservedCodeCacheSize`）。
- 【补充】`-Xint` 仅解释、`-Xcomp` 仅编译；逃逸分析、标量替换、锁消除、栈上分配等是 JIT 重要优化。

### 2.4 垃圾回收（GC）

#### 对象存活判定

- **引用计数法**：简单但有循环引用问题，JVM 不采用。
- **可达性分析**：从 **GC Roots** 出发遍历，不可达即可回收。
  - GC Roots 包括：虚拟机栈中局部变量表引用的对象、方法区静态属性/常量引用的对象、本地方法栈 JNI 引用的对象、活跃线程、同步锁对象等。
- 引用类型：强引用（不回收）、软引用（内存不足时回收，适合缓存）、弱引用（下次 GC 必回收）、虚引用（仅用于回收通知）。

#### GC 算法

- 标记-清除：产生内存碎片。
- 复制算法：新生代（Eden/Survivor），空间换效率但浪费空间。
- 标记-整理：老年代，无碎片但移动对象开销大。
- 分代收集：新生代用复制，老年代用标记-清除/整理。

#### 垃圾收集器

| 收集器 | 代 | 算法 | 特点 |
|---|---|---|---|
| Serial | 新生代 | 复制 | 单线程，STW；Client/单核 |
| ParNew | 新生代 | 复制 | Serial 多线程版，常与 CMS 搭配 |
| Parallel Scavenge | 新生代 | 复制 | 多线程，追求**吞吐量** |
| Serial Old | 老年代 | 标记-整理 | Serial 老年代版，CMS 后备 |
| Parallel Old | 老年代 | 标记-整理 | 与 Parallel Scavenge 搭配 |
| CMS | 老年代 | 标记-清除 | **最短停顿**；四阶段：初始标记(STW)、并发标记、重新标记(STW)、并发清除；有碎片、并发模式失败 |
| G1 | 整堆 | 复制+标记-整理（Region） | 可预测停顿、按 Region 回收，JDK9 起默认 |
| ZGC | 整堆 | 着色指针+读屏障 | 超低停顿(≤10ms)，适合超大堆 |
| Shenandoah | 整堆 | 并发整理 | 低停顿，与 ZGC 类似 |

- **各 JDK 默认收集器【修正】**：
  - JDK7：Parallel Scavenge + Parallel Old。
  - **JDK8（Server 模式）：Parallel Scavenge + Parallel Old**（原文误写为「+ Serial Old」，已更正）。
  - JDK9 起：G1。
- 【补充】CMS 在 JDK9 被标记废弃、JDK14 移除；G1 适合大堆、低延迟；ZGC 适合超大堆极低停顿。STW = Stop The World。
- 【补充】Minor GC / Major GC / Full GC；`System.gc()` 只是建议；对象优先在 Eden 分配，大对象直接进老年代，长期存活对象晋升老年代；动态年龄判定；空间分配担保。

### 2.5 Compressed Oops / 其他优化参数【补充】

- 压缩指针（Compressed Oops）：64 位平台用 32 位指针引用对象，减少内存占用（堆 < 32G 时默认开启 `-XX:+UseCompressedOops`）。
- `Compressed Class Space`：存放类元数据的压缩指针区域，类加载过多可能报 `OutOfMemoryError: Compressed class space`。
- `-XX:+UseG1GC -XX:+UseStringDeduplication`：G1 + 字符串去重。
- `-XX:ParallelGCThreads`：并行 GC 线程数；`-XX:+UseCGroupMemoryLimitForHeap`（JDK8u131+）让 JVM 感知容器 cgroup 限制。
- Code Cache 清理：`-XX:+MethodFlushing` 触发清理不用的编译代码。

---

## 三、Java 并发编程

### 3.1 线程基础

- 线程是 **CPU 调度的最小单位**，进程是**资源分配的基本单位**；线程不能独立存在，依附于进程；Java 中 main 所在线程为普通用户线程。
- **创建方式**：
  - 继承 `Thread` 重写 `run`（缺点：单继承限制、任务与线程耦合、无返回值）。
  - 实现 `Runnable`（可共享任务代码、避免单继承限制，无返回值）。
  - 实现 `Callable` + `FutureTask`（**有返回值、可抛异常**）。
  - 【补充】线程池创建（推荐）、`CompletableFuture`。
- **生命周期**：新建（New）→ 就绪（Runnable）→ 运行（Running）→ 阻塞（Blocked/Waiting/Timed_Waiting）→ 终止（Terminated）。`start()` 后才进入就绪态等待 CPU。
- **核心方法**：
  - `wait()/notify()/notifyAll()`：属于 Object，必须在 synchronized 内使用，释放锁。
  - `sleep()`：Thread 静态方法，不释放锁。
  - `yield()`：让出 CPU 回到就绪态，可能被忽略。
  - `join()`：等待目标线程结束。
  - `interrupt()`：协作式中断，配合 `isInterrupted()`/`interrupted()` 使用。
  - 【补充】`wait()` 应放在 `while` 循环中防止**虚假唤醒**。
- **死锁四条件**：互斥、请求并持有、不可剥夺、循环等待；破坏「请求并持有」或「循环等待」（资源有序分配）可避免死锁。
- **守护线程（Daemon）**：随用户线程结束而结束（如 GC 线程）；`setDaemon(true)` 需在 `start()` 前设置。

### 3.2 ThreadLocal

- 为每个线程提供独立的变量副本，底层是线程内部的 `ThreadLocalMap`（key 为 ThreadLocal 弱引用，value 强引用）。
- 常用于保存用户上下文、数据库连接、`SimpleDateFormat` 等。
- **内存泄漏风险【补充】**：key 是弱引用会被回收，但 value 是强引用；线程池中线程复用且不 `remove()` 会导致 value 无法回收，最终可能 OOM。**用完务必 `remove()`**。
- `InheritableThreadLocal` 可让子线程继承父线程的值（线程池场景下继承会失效，需 TransmittableThreadLocal）。

### 3.3 Java 内存模型（JMM）

- 三大特性：**原子性、可见性、有序性**。
- JMM 规定所有变量存于主内存，线程操作的是工作内存副本；共享变量在多核 Cache 下会出现**内存可见性问题**。
- **happens-before 规则【补充】**：程序顺序、监视器锁（unlock 先于 lock）、volatile（写先于读）、线程启动/终止、传递性等。
- **指令重排序**：编译器/CPU 在不影响单线程语义前提下重排；用 `volatile`/内存屏障禁止关键重排。
- `volatile`：保证**可见性**和**有序性（禁止重排）**，**不保证原子性**；适合状态标志位、双重检查锁的单例。
- `synchronized`：保证原子性、可见性、有序性；进入时清空工作内存、退出时刷新主内存。
- 【补充】原子性操作：除 long/double 外基本类型赋值、引用赋值、`java.util.concurrent.atomic.*` 的操作。

### 3.4 CAS 与原子类

- CAS（Compare And Swap）：`Unsafe.compareAndSwapXxx`，基于硬件保证原子性；属于乐观锁/无锁。
- **ABA 问题**：用版本号/时间戳解决（`AtomicStampedReference`）。
- `AtomicLong` 高并发下所有线程竞争同一变量，CAS 失败自旋浪费 CPU；`LongAdder` 将热点分散到多 Cell，高并发吞吐更高，但统计可能有误差；`LongAccumulator` 支持自定义累加规则。
- 【补充】`Unsafe` 提供底层 CAS、`park/unpark`、内存偏移等能力，不建议直接使用。

### 3.5 锁的分类

- 按是否加锁：**乐观锁 / 悲观锁**。
- 按竞争是否排队：**公平锁 / 非公平锁**（非公平吞吐高但可能饥饿）。
- 按能否共享：**共享锁 / 独占锁**（读写锁）。
- 按是否可重入：**可重入锁 / 不可重入锁**。
- 按是否阻塞：**自旋锁 / 适应性自旋锁 / 阻塞锁**。

#### synchronized 锁升级【补充 / 修正】

- 状态演进：**无锁 → 偏向锁 → 轻量级锁 → 重量级锁**（只能升级不能降级）。
  - 偏向锁：同一线程首次进入，把线程 ID 记入对象头 Mark Word（CAS），后续无竞争直接进入。
  - 轻量级锁：出现竞争时撤销偏向，线程用 CAS 自旋竞争对象头锁记录。
  - 重量级锁：自旋失败或竞争加剧，膨胀为重量级锁，依赖操作系统互斥量（Monitor），线程阻塞。
- 【修正】偏向锁在 **JDK15 被标记废弃、JDK18 默认禁用**，新版本实际从轻量级锁起步。

### 3.6 AQS（AbstractQueuedSynchronizer）

- 并发包锁与同步器的底层基础，核心是 **volatile int state + CLH 双向队列（FIFO）+ CAS**。
- 线程通过 CAS 修改 state 抢占；失败则封装成 Node 入队并阻塞（`LockSupport.park`），前驱释放时唤醒。
- Node 关键字段：`waitStatus`（0 初始化 / 1 取消 / -1 SIGNAL / -2 CONDITION / -3 PROPAGATE）、`prev/next`、`thread`。
- **模板方法**：独占方式 `tryAcquire/tryRelease`、共享方式 `tryAcquireShared/tryReleaseShared`。
- `ConditionObject` 内部类实现条件队列，`await/signal` 类似 `wait/notify`，必须在 lock/unlock 之间使用。
- **ReentrantLock**：基于 AQS 的可重入独占锁；state=0 空闲，≥1 表示重入次数；默认非公平（可通过构造参数设公平）。
- **ReentrantReadWriteLock**：用 state 高 16 位记录读锁数量、低 16 位记录写锁重入次数，读读共享、读写互斥；读多写少场景，写线程可能饥饿。
- **StampedLock【补充】**：JDK8 引入，支持写锁、悲观读锁、乐观读锁，性能更好；乐观读不加锁，需校验 stamp；不可重入。

### 3.7 并发容器与阻塞队列

- `ConcurrentHashMap`：**JDK7 用 Segment 分段锁；JDK8 改为「CAS + synchronized 锁单个桶头节点」**，并引入红黑树，锁粒度更细、并发更高。key/value 不允许 null。
- `CopyOnWriteArrayList`：写时复制（写加 ReentrantLock 复制新数组），读无锁、适合读多写少；弱一致性（读可能读到旧值）。
- `ConcurrentLinkedQueue`：无界非阻塞队列，CAS 实现（`add/poll` 线程安全）。
- 阻塞队列：
  - `ArrayBlockingQueue`：有界数组，FIFO。
  - `LinkedBlockingQueue`：链表，默认长度 `Integer.MAX_VALUE`（近似无界）。
  - `PriorityBlockingQueue`：优先级队列，不阻塞生产者，可能耗尽内存。
  - `DelayQueue`：延迟到期才能取出，用于订单超时取消、定时任务、缓存过期等。
  - `SynchronousQueue`：不存储元素，配对传递。

### 3.8 线程池 ThreadPoolExecutor

- **七大参数**：`corePoolSize`（核心线程数）、`maximumPoolSize`（最大线程数）、`keepAliveTime`（空闲存活时间）、`unit`（时间单位）、`workQueue`（任务队列）、`threadFactory`（线程工厂）、`handler`（拒绝策略）。
- **执行流程**：核心线程未满 → 建核心线程；核心线程满 → 入队；队列满 → 建非核心线程至最大；仍满 → 触发拒绝策略。
- **拒绝策略**：`AbortPolicy`（抛异常，默认）、`CallerRunsPolicy`（调用者线程执行）、`DiscardPolicy`（丢弃）、`DiscardOldestPolicy`（丢最旧）。
- **四种常见线程池**（Executors，**生产不建议**）：
  - `FixedThreadPool`：固定核心线程，队列为无界 `LinkedBlockingQueue`（可能 OOM）。
  - `SingleThreadExecutor`：单线程，无界队列。
  - `CachedThreadPool`：核心 0、最大 `Integer.MAX_VALUE`、`SynchronousQueue`（可能创建大量线程）。
  - `ScheduledThreadPool`（`ScheduledThreadPoolExecutor`）：定时/周期任务。
- 【补充 / 修正】阿里规约不建议用 Executors 创建线程池，应手动 new `ThreadPoolExecutor` 并**指定有界队列 + 有意义的线程名 + 合理拒绝策略**；`keepAliveTime` 对核心线程默认不生效（需 `allowCoreThreadTimeOut(true)`）。
- 关闭：`shutdown()`（不再接新任务、执行完已提交任务）、`shutdownNow()`（尝试中断、返回未执行任务）。
- 结果获取：`submit()` 返回 `Future`，`Future.get()` 阻塞取结果；异常通过 `Future.get()` 抛出 `ExecutionException`，或覆写 `afterExecute` 捕获。
- `ThreadLocalRandom`：多线程下生成随机数避免 `Random` 的种子 CAS 竞争，性能更好。
- **同步器**：`CountDownLatch`（一次性，倒计数等待）、`CyclicBarrier`（可循环，屏障点汇合）、`Semaphore`（信号量限流）、`Exchanger`（两线程交换数据）。

### 3.9 并发编程注意事项【补充】

- `SimpleDateFormat` 线程不安全（内部 Calendar），应每线程独立或用 `DateTimeFormatter`。
- `Timer` 任务抛异常会终止整个定时线程，建议用 `ScheduledExecutorService`。
- 线程池任务中务必 try-catch，避免异常吞没；用 FutureTask 注意 `get` 阻塞与取消。
- 创建线程/线程池指定业务相关名称便于排查；程序结束记得 `shutdown`。
- 伪共享：多个变量落在同一 Cache Line 互相影响性能，可用 `@sun.misc.Contended` 或字节填充解决。

### 3.10 CompletableFuture【补充】

- `Future` 的 `get()` 阻塞、不能组合回调；`CompletableFuture` 支持异步编排。
- 常用：`supplyAsync`/`runAsync`（异步执行）、`thenApply`/`thenAccept`/`thenCompose`（串行）、`thenCombine`（合并两个结果）、`allOf`/`anyOf`（聚合多个）。
- 异常：`exceptionally`/`handle`/`whenComplete` 处理；可指定线程池，避免默认 `ForkJoinPool.commonPool()` 阻塞。
- 【补充】`CompletableFuture` 是真正的异步非阻塞编排工具，适合并行聚合 RPC 调用。

---

## 四、Spring 与 Spring Boot

### 4.1 Spring 核心模块

- **Core Container（核心容器）**
  - `spring-core`：核心工具类，其它模块的基础。
  - `spring-beans`：IoC/DI，BeanFactory 与 Bean 的定义、创建、管理。
  - `spring-context`：建立在 core/beans 之上，提供国际化、事件传播、资源加载、`ApplicationContext`。
  - `spring-expression`：SpEL 表达式。
- **AOP**：`spring-aop`、`spring-aspects`、`spring-instrument`。
- **数据访问/集成**：`spring-jdbc`、`spring-tx`、`spring-orm`、`spring-oxm`、`spring-jms`。
- **Web**：`spring-web`、`spring-webmvc`、`spring-websocket`、`spring-webflux`。

- **BeanFactory 体系【补充】**：`BeanFactory`（顶层）→ `HierarchicalBeanFactory` / `ListableBeanFactory` / `AutowireCapableBeanFactory` / `ConfigurableBeanFactory` → `DefaultListableBeanFactory`（最终默认实现，综合注册与获取）。
- IoC：控制反转，把对象创建与依赖管理交给容器；DI：依赖注入（构造器/Setter/字段），是 IoC 的实现方式。

### 4.2 Bean 生命周期【补充 / 修正】

1. 实例化（反射调用构造器）
2. 属性填充（依赖注入）
3. `Aware` 接口回调（BeanNameAware、BeanFactoryAware、ApplicationContextAware 等）
4. `BeanPostProcessor.postProcessBeforeInitialization`
5. 初始化：`@PostConstruct` → `InitializingBean.afterPropertiesSet` → `init-method`
6. `BeanPostProcessor.postProcessAfterInitialization`（**AOP 代理通常在此生成**）
7. 使用
8. 销毁：`@PreDestroy` → `DisposableBean.destroy` → `destroy-method`

- Bean 作用域：`singleton`（默认）、`prototype`、`request`、`session`、`application`、`websocket`。

### 4.3 循环依赖与三级缓存【修正】

- Spring 用**三级缓存**解决**单例 Bean 的字段/Setter 注入**循环依赖：
  - 一级 `singletonObjects`：成品单例池。
  - 二级 `earlySingletonObjects`：提前暴露的半成品 Bean。
  - 三级 `singletonFactories`：`ObjectFactory` 工厂（可在此生成 AOP 代理）。
- 流程简述：A 实例化后先把工厂放入三级缓存 → 注入 B → B 注入 A 时从三级缓存拿到 A 的早期引用（必要时提前生成代理）→ B 完成 → A 完成。
- **限制【补充/修正】**：只能解决**单例 + Setter/字段注入**；**构造器注入的循环依赖无法解决**（报错）；`prototype` 作用域无法解决；`@Async` 等代理场景需注意。

### 4.4 AOP

- 核心概念：切面（Aspect）、切点（Pointcut）、通知（Advice：Before/After/AfterReturning/AfterThrowing/Around）、连接点（JoinPoint）、织入（Weaving）。
- 底层：JDK 动态代理（目标实现接口）与 CGLIB（无接口，生成子类）；Spring Boot 2.x 起默认 CGLIB。
- 应用：事务、日志、权限、缓存等横切关注点。

### 4.5 Spring 事务

- 隔离级别：DEFAULT、READ_UNCOMMITTED、READ_COMMITTED、REPEATABLE_READ、SERIALIZABLE（MySQL 默认 RR）。
- 传播行为：REQUIRED（默认）、REQUIRES_NEW、NESTED、SUPPORTS、NOT_SUPPORTED、MANDATORY、NEVER。
- 【补充】`@Transactional` 失效场景：同类内部方法自调用（绕过代理）、方法非 public、异常被 catch 未抛出、抛出受检异常但未设 `rollbackFor`、数据库引擎不支持事务。

### 4.6 Spring Boot

- **启动流程（两大步）**：
  1. 创建 `SpringApplication`：推断应用类型（Web/非 Web）、加载 `ApplicationContextInitializer`、加载 `ApplicationListener`、推断主配置类。
  2. 运行 `run()`：准备环境（Environment，加载命令行参数/系统属性/配置文件）→ 创建 `ApplicationContext` → `refresh()`（IoC 刷新、自动配置、Bean 扫描装配、启动 Web 服务器）→ 启动后扩展点、发布事件 → 启动完成/报错。
- **自动配置原理**：`@SpringBootApplication` = `@SpringBootConfiguration` + `@EnableAutoConfiguration` + `@ComponentScan`。
  - `@EnableAutoConfiguration` → `@Import(AutoConfigurationImportSelector)` 导入自动配置类。
  - 【修正】加载来源：**Spring Boot 2.7 之前为 `META-INF/spring.factories`；2.7+ / 3.0 改为 `META-INF/spring/org.springframework.boot.autoconfigure.AutoConfiguration.imports`**。
  - 条件注解（`@ConditionalOnClass/@ConditionalOnMissingBean` 等）控制是否生效；自动配置类通常读取配置文件参数。
- 配置文件优先级（从高到低，常见）：命令行参数 > `application-{profile}.properties/yml` > `application.properties/yml` > jar 内部 > jar 外部（不同位置有细分规则）。
- 可执行 Jar：Spring Boot 使用自定义 `LaunchedURLClassLoader` 加载 `BOOT-INF/classes` 与 `BOOT-INF/lib`，普通 jar 由 `AppClassLoader` 直接加载。
- 【补充】`spring.factories` 还用于注册 `EnvironmentPostProcessor`、`ApplicationListener` 等扩展；自定义 Starter = 自动配置类 + `@ConfigurationProperties` + imports 文件。

### 4.7 Spring MVC 请求执行流程【补充】

1. 请求到达 `DispatcherServlet`（前端控制器）。
2. `HandlerMapping` 根据 URL 找到处理器（Controller 方法）及拦截器，返回 `HandlerExecutionChain`。
3. `HandlerAdapter` 调用处理器方法（参数解析：`HandlerMethodArgumentResolver`）。
4. 返回 `ModelAndView`；`ViewResolver` 解析视图名（或 `@ResponseBody` 由 `HttpMessageConverter` 直接序列化为 JSON）。
5. 视图渲染（或消息转换写回）。
6. `interceptor.postHandle` → 渲染 → `afterCompletion`；异常由 `HandlerExceptionResolver` 处理。

- 【补充】常用注解：`@Controller/@RestController`、`@RequestMapping`、`@RequestParam/@PathVariable/@RequestBody`、`@ResponseBody`。

---

## 五、MySQL 与数据库

### 5.1 索引分类

**按表列属性**：
- 单列索引、联合索引（复合索引）。
- 普通索引（无约束，仅加速查找）、主键索引（唯一且非空）、唯一索引（不重复、允许 null）、全文索引（全文检索，`like %关键字%` 低效时的替代）。

**按数据结构**：
- **B+Tree**（InnoDB 默认）：多路平衡查找树，非叶子节点只存键、叶子节点存数据并用链表相连，适合范围查询与排序。
- **Hash**：等值查找 O(1)，不支持范围查找/排序/最左匹配；仅 MEMORY/NDB 支持显式 hash 索引，InnoDB 有「自适应哈希索引」（自动、不可人为定义）。
- R-Tree（空间索引）、T-Tree（内存数据库常见）。

**按存储结构**：
- **聚簇索引（聚集索引）**：索引与整行数据存于同一 B+Tree，叶子节点即数据页；一张表只有一个，通常就是主键。
- **辅助索引（二级/非聚集索引）**：叶子节点存索引列 + 主键值；查非索引列需**回表**。

### 5.2 聚簇索引的优缺点【补充】

- 优点：相关数据物理相邻、范围查询快；主键查找可直接拿到整行；覆盖索引扫描可利用叶子节点主键值。
- 缺点：插入速度依赖主键顺序（乱序插入慢，建议插入后 `OPTIMIZE TABLE`）；更新主键代价高；页分裂导致碎片与空间浪费；全表扫描可能变慢；二级索引需两次查找（回表）。

### 5.3 索引使用与失效【补充】

- **最左匹配原则**：联合索引 `(a,b,c)` 查询条件需从最左列开始连续匹配；`where a=1 and c=3` 只能用 a。
- **覆盖索引**：查询列都在索引中则无需回表，`explain` 显示 `Using index`。
- **索引下推（ICP）**：MySQL 5.6+ 在索引遍历时提前用索引列过滤，减少回表（`Using index condition`）。
- **索引失效常见场景**：
  - 违反最左前缀；在索引列上做运算/函数；隐式类型转换（字符串列传数字）；`like '%xx'` 前导模糊；`or` 两边未都建索引；使用 `!=`/`not in` 可能失效；范围查询后列失效；数据区分度太低（如性别）；使用 `select *` 导致回表成本高。
- `explain` 关键列：`type`（system>const>eq_ref>ref>range>index>ALL）、`key`（实际用到的索引）、`rows`（预估扫描行数）、`Extra`（Using index/where/filesort/temporary 等）。

### 5.4 事务 ACID 与实现

- **ACID**：原子性（Atomicity）、一致性（Consistency）、隔离性（Isolation）、持久性（Durability）。
- 实现机制【修正】：
  - **undo log**：记录数据旧版本，用于**回滚**和 **MVCC 读**。
  - **redo log**：物理日志，记录页修改，用于**崩溃恢复**（保证持久性）。
  - **binlog**：Server 层逻辑日志，记录所有修改语句，用于**主从复制与数据恢复**；与 redo 两阶段提交保证一致性。
  - **MVCC**：通过隐藏列（trx_id、roll_pointer）+ undo log + ReadView 实现一致性非锁定读。

### 5.5 隔离级别与并发问题

| 隔离级别 | 脏读 | 不可重复读 | 幻读 |
|---|---|---|---|
| 读未提交 READ UNCOMMITTED | 可能 | 可能 | 可能 |
| 读已提交 READ COMMITTED | 否 | 可能 | 可能 |
| 可重复读 REPEATABLE READ | 否 | 否 | InnoDB 基本避免* |
| 串行化 SERIALIZABLE | 否 | 否 | 否 |

- MySQL InnoDB 默认 **REPEATABLE READ**；`*` 通过 MVCC 快照读 + 间隙锁（Next-Key Lock）在很大程度避免幻读，但快照读与当前读混用仍可能出现。
- Oracle/SQL Server 默认 READ COMMITTED（原文「Oracle 默认提交读」正确）。

### 5.6 InnoDB 锁【补充】

- 按粒度：表锁、行锁。
- 行锁类型：记录锁（Record Lock）、间隙锁（Gap Lock）、临键锁（Next-Key Lock = 记录锁 + 间隙锁，RR 默认）。
- 意向锁：意向共享锁（IS）、意向排他锁（IX）。
- 共享锁（S）/排他锁（X）；`select ... for update` 加 X 锁，`lock in share mode` 加 S 锁。
- 死锁：InnoDB 会检测并回滚代价较小的事务；预防：按固定顺序访问资源、缩短事务、合理索引减少锁范围。

### 5.7 InnoDB 体系结构

- **内存**：
  - 缓冲池 `innodb_buffer_pool_size`：缓存数据页/索引页（16KB 页），LRU 管理，是性能最关键参数。
  - 重做日志缓冲 `innodb_log_buffer_size`。
  - 额外内存池 `innodb_additional_mem_pool_size`。
  - 【补充】还包含 change buffer（写缓冲）、自适应哈希索引、log buffer 等。
- **后台线程**：Master Thread（核心调度）、IO Thread、Purge Thread、Page Cleaner Thread。
- 查看：`show engine innodb status;`。

### 5.8 存储引擎对比

| 引擎 | 事务 | 锁 | 特点 |
|---|---|---|---|
| InnoDB | 支持 | 行锁 | 支持外键、MVCC、崩溃恢复，默认引擎 |
| MyISAM | 不支持 | 表锁 | 支持全文索引，读快，崩溃难恢复 |
| Memory | 不支持 | 表锁 | 数据在内存，重启丢失，hash 索引 |
| NDB | 支持 | 行锁 | 集群引擎，数据内存，JOIN 网络开销大 |
| Archive | 不支持 | 行锁 | 只支持插入/查询，高压缩 |
| Federated | 不支持 | — | 不存数据，指向远程表，不支持异构 |
| Maria | 支持 | 行锁 | MyISAM 的升级，支持 MVCC |

### 5.9 常用参数

- 连接类：`max_connections`（最大连接数）、`max_user_connections`、`back_log`（连接等待队列长度）、`wait_timeout`/`interactive_timeout`。
- 缓存类：`innodb_buffer_pool_size`（最重要）、`key_buffer_size`（MyISAM 索引缓存）、`query_cache_size`（8.0 已移除查询缓存）。
- 日志类：`innodb_log_buffer_size`、`innodb_log_file_size`、`binlog`、慢查询日志 `slow_query_log`。
- 会话/排序：`sort_buffer_size`、`read_buffer_size`、`read_rnd_buffer_size`、`join_buffer_size`、`tmp_table_size`、`max_heap_table_size`。
- 线程：`thread_cache_size`（1G→8,2G→16,3G→32,>3G→64 经验值）。
- 【补充】`innodb_buffer_pool_size` 一般设为物理内存的 50%~70%；命中率 = (read_requests - reads) / read_requests。

### 5.10 SQL 执行与优化【补充】

- 执行流程：连接器 → 查询缓存（8.0 移除）→ 解析器 → 预处理器 → 优化器（生成执行计划）→ 执行器 → 存储引擎。
- 慢查询：开启 `slow_query_log`，用 `explain`/`optimizer trace`/profiling 分析。
- 优化方向：合理建索引、避免 `select *`、避免隐式转换、分页优化（延迟关联/`where id > ?`）、`join` 小表驱动大表、`order by/group by` 走索引、减少回表、读写分离、分库分表。
- 深分页优化：`limit 100000,10` 慢，可改为「延迟关联」或基于上次最大 id 的游标分页。

---

## 六、Redis

### 6.1 数据类型

- 基础五类：**String、Hash、List、Set、ZSet（Sorted Set）**。
- 高级类型【补充】：Bitmap（位图）、HyperLogLog（基数统计）、GEO（地理位置）、Stream（消息流）、Pub/Sub（发布订阅）。
- 典型用途：String（缓存/计数器/分布式锁）、List（队列/栈）、Hash（对象存储）、Set（去重/交并差）、ZSet（排行榜/延时队列）。

### 6.2 持久化

- **RDB**：周期性快照（`save`/`bgsave`），文件小、恢复快，但可能丢最后一次快照后的数据。
- **AOF**：追加写命令日志，实时性高；`appendfsync` 策略：`always`（最安全最慢）、`everysec`（默认，最多丢 1 秒）、`no`（交给 OS）。
- 【修正/补充】Redis 4.0+ 支持 **RDB + AOF 混合持久化**；AOF 重写（rewrite）压缩日志体积；RDB 适合备份/全量恢复，AOF 适合增量/更好持久性。

### 6.3 缓存三大问题

- **缓存穿透**：缓存和数据库都没有的数据（如恶意查不存在的 id）。方案：缓存空值 / 布隆过滤器 / 接口校验。
- **缓存击穿**：某个**热点 key 过期**瞬间大量请求打到数据库。方案：热点 key 永不过期 / 互斥锁重建。
- **缓存雪崩**：**大量 key 同时过期**或 Redis 宕机导致请求全压 DB。方案：过期时间加随机值、集群高可用、限流降级、多级缓存。

### 6.4 分布式锁与队列

- **分布式锁**：`SET key value NX EX`（setnx+expire 合并的原子操作）；释放时需校验 value（Lua 保证原子）；进阶用 **Redisson**（可重入、看门狗自动续期）、RedLock。
- **异步队列**：`List` 结构 `rpush + lpop/blpop`（`blpop` 阻塞）；`Pub/Sub` 支持 1:N 但消费者下线会丢消息，可靠场景用专业 MQ。
- **延时队列**：用 `ZSet`，score 为到期时间戳，消费者轮询 `zrangebyscore` 取到期任务。
- **Pipeline**：批量发送命令减少 RTT，要求命令间无因果依赖。

### 6.5 集群与高可用

- **主从复制**：读写分离，从节点同步主节点数据。
- **哨兵（Sentinel）**：监控、自动故障转移（master 宕机提升 slave）。
- **Cluster**：分片存储（16384 个 hash slot），解决单机内存上限，支持水平扩展。
- 【补充】Redis 单线程指**命令执行单线程**（避免锁竞争、基于内存和 IO 多路复用高效）；**Redis 6.0 引入多线程处理网络 IO**，命令执行仍单线程。

### 6.6 其他高频问题

- 为什么快：纯内存、IO 多路复用（epoll）、单线程无锁、高效数据结构（跳表、SDS、ziplist/quicklist/intset/listpack）。
- 内存淘汰策略：`noeviction`、`allkeys-lru`、`volatile-lru`、`allkeys-random`、`volatile-random`、`volatile-ttl`、`allkeys-lfu`、`volatile-lfu`。
- 缓存与数据库一致性：常用「先更新库再删缓存」（Cache-Aside）+ 延迟双删 / 订阅 binlog（Canal）异步失效。
- 跳表（skiplist）：ZSet 底层，多层链表，查找/插入/删除平均 O(logN)，实现比红黑树简单。
- `INCR` 可做分布式自增 id（需考虑持久化与单点）。
- 内存溢出处理：扩容、设置 maxmemory + 淘汰策略、分析大 key/热 key、拆分。

---

## 七、Netty 与 NIO

### 7.1 IO 模型

- **BIO**（同步阻塞）：一连接一线程，简单但并发差、资源消耗大。
- **NIO**（同步非阻塞）：基于 Channel + Buffer + Selector，IO 多路复用，一个线程管理多个连接。
- **AIO**（异步非阻塞）：基于回调/ Future，操作完成后通知，NIO.2。

### 7.2 select / poll / epoll【补充】

- `select`：轮询 fd 集合，有最大连接数限制（1024），每次需把 fd 从用户态拷贝到内核态，O(n) 扫描。
- `poll`：链表结构，无最大连接数限制，仍需 O(n) 扫描与拷贝。
- `epoll`：内核事件表 + 回调，`epoll_ctl` 注册、`epoll_wait` 只返回就绪 fd，O(1) 就绪感知，适合高并发；有 LT（水平触发）与 ET（边缘触发）模式。

### 7.3 Netty 核心组件

- **Channel**：网络连接抽象（NioSocketChannel / NioServerSocketChannel 等）。
- **EventLoop / EventLoopGroup**：事件循环与线程组，一个 EventLoop 绑定一个线程、处理多个 Channel；bossGroup 负责 accept，workerGroup 负责读写。
- **ChannelPipeline / ChannelHandler / ChannelHandlerContext**：责任链处理入站/出站事件。
- **ByteBuf**：增强的字节缓冲区，支持池化、读写双指针、零拷贝（CompositeByteBuf、FileRegion）。
- **Unsafe**：Channel 内部底层操作（如注册、绑定）。
- **Promise / Future**：异步结果通知。
- **Reactor 线程模型**：单 Reactor 单线程 / 单 Reactor 多线程 / 主从 Reactor 多线程（Netty 常用主从）。

### 7.4 常见问题【补充】

- **TCP 粘包/拆包**：解决方式——固定长度、分隔符（DelimiterBasedFrameDecoder）、长度字段（LengthFieldBasedFrameDecoder）、消息定长/特殊协议。
- **JDK NIO 空轮询 Bug**：epoll 空轮询导致 CPU 100%；Netty 通过统计 select 次数（默认 512 次）后重建 Selector 规避。
- **零拷贝**：堆外直接内存 + `FileRegion`（transferTo）+ CompositeByteBuf 减少数据拷贝。

---

## 八、计算机网络

### 8.1 分层与各层要点

- **应用层**：HTTP、HTTPS、DNS、FTP、SMTP 等。
- **传输层**：TCP、UDP；端到端通信；端口号（熟知 0~1023、登记 1024~49151、动态 49152~65535）。
- **网络层**：IP、ARP、ICMP、IGMP、路由（RIP/OSPF/BGP）；主机间逻辑通信、路由与转发、拥塞控制。
- **数据链路层**：帧封装、透明传输、差错控制（CRC/海明码）、流量控制（停等/滑动窗口）、介质访问控制（CSMA/CD、CSMA/CA）、以太网/PPP。
- **物理层**：比特流传输、机械/电气/功能/规程特性、奈氏准则与香农公式、编码与调制。

### 8.2 TCP 核心

- **特点**：面向连接、可靠交付、全双工、面向字节流、点对点、有流量控制与拥塞控制。套接字 socket = (IP: 端口)。
- **三次握手**：`SYN(x)` → `SYN(y)+ACK(x+1)` → `ACK(y+1)`。目的：同步双方初始序号、确认收发能力；第三次握手防止**失效的历史连接请求**造成服务器资源浪费。
- **四次挥手**：`FIN` → `ACK`（半关闭）→ `FIN` → `ACK`；主动关闭方进入 **TIME_WAIT** 等待 **2MSL**（确保最后 ACK 可达 + 旧报文消散）。被动关闭方先进入 CLOSE_WAIT。
- **可靠传输**：序号/确认、超时重传（RTO 略大于加权平均 RTT）、滑动窗口、选择确认 SACK。
- **流量控制**：接收方通过窗口字段控制发送方速率（窗口为 0 时停发）。
- **拥塞控制**【修正】：慢开始（cwnd 从 1 指数增长至 ssthresh）→ 拥塞避免（cwnd 线性增长）→ 超时/丢包时 `ssthresh = cwnd/2` 并重新慢开始；**快重传**（收到 3 个重复 ACK 立即重传）→ **快恢复**（`ssthresh = cwnd/2`，`cwnd = ssthresh`，直接进入拥塞避免）。「慢开始/快恢复」指 cwnd 的起始设定值，而非增长速度。
- **UDP**：无连接、尽最大努力交付、无拥塞控制、面向报文、支持一对一/一对多/多对多。

### 8.3 网络层

- IPv4 地址分类、子网划分、CIDR（超网）；NAT 私有/公网地址转换。
- ARP（IP↔MAC）、DHCP（动态分配 IP）、ICMP（差错/询问，ping/traceroute）、IGMP（组播）。
- 路由算法：静态/动态；链路状态（OSPF）、距离向量（RIP）；外部网关（BGP）。
- IPv6：128 位、无校验和、扩展首部、即插即用；过渡技术：双栈、隧道。

### 8.4 HTTP / HTTPS【补充】

- HTTP 方法：GET/POST/PUT/DELETE 等；状态码：2xx 成功、3xx 重定向、4xx 客户端错误、5xx 服务端错误。
- HTTP/1.1 长连接、HTTP/2 多路复用/头部压缩、HTTP/3 基于 QUIC（UDP）。
- HTTPS = HTTP + TLS，非对称加密协商 + 对称加密传输 + 证书校验，保证机密性、完整性、身份认证。

---

## 九、Linux 命令与 Shell

### 9.1 常用命令分类（精选）

- **查看帮助**：`man`、`help`、`info`。
- **文件/目录**：`ls`、`cd`、`pwd`、`cp`、`mv`、`rm`、`mkdir`、`rmdir`、`touch`、`find`、`tree`、`basename`、`dirname`。
- **查看文件内容**：`cat`、`tac`、`more`、`less`、`head`、`tail`（`tail -f`）、`cut`、`sort`、`uniq`、`wc`、`diff`、`grep/egrep`、`tr`、`vi/vim`。
- **压缩解压**：`tar`、`gzip`、`zip/unzip`。
- **系统信息**：`uname`、`hostname`、`uptime`、`du`、`df`、`top`、`free`、`date`、`stat`。
- **搜索**：`which`、`whereis`、`locate`、`find`。
- **用户管理**：`useradd`、`usermod`、`userdel`、`groupadd`、`passwd`、`su`、`sudo`、`visudo`、`id`。
- **网络**：`ping`、`telnet`、`ssh`、`scp`、`wget`、`ifconfig/ip`、`netstat/ss`、`route`、`traceroute`、`nslookup/dig/host`、`tcpdump`、`lsof`、`nmap`。
- **权限**：`chmod`、`chown`、`chgrp`、`umask`。
- **登录信息**：`whoami`、`who`、`w`、`last`、`lastlog`、`users`。
- **进程管理**：`ps`、`pstree`、`top`、`kill`、`killall`、`pkill`、`jobs`、`bg`、`fg`、`nohup`、`nice/renice`、`crontab`。
- **系统管理/性能**：`chkconfig`、`systemctl`、`service`、`vmstat`、`iostat`、`mpstat`、`sar`、`strace`、`ipcs`。
- **磁盘文件系统**：`mount`、`umount`、`fsck`、`dd`、`fdisk`、`parted`、`mkfs`、`mkswap`、`swapon/swapoff`、`sync`、`resize2fs`。
- **关机重启**：`shutdown`、`halt`、`poweroff`、`reboot`、`logout`、`exit`。

### 9.2 Shell 脚本要点【补充】

- 变量：`VAR=value`（等号两侧不能有空格），使用 `$VAR`/`${VAR}`；`export` 导出环境变量。
- 特殊变量：`$0` 脚本名、`$1..$9` 参数、`$#` 参数个数、`$?` 上条命令退出码、`$$` 当前进程 PID。
- 条件与循环：`if [ ... ]; then ...; fi`、`for`、`while`；`test`/`[ ]`/`[[ ]]`。
- 常用：`$( )` 命令替换、管道 `|`、重定向 `>` `>>` `<` `2>&1`、`&&` `||`。
- 排查思路：`tail -f` 看日志、`grep` 过滤、`ps/top` 查进程、`netstat/ss` 查端口、`df/du` 查磁盘、`jstack/jmap` 查 Java。

---

## 十、设计模式

### 10.1 七大设计原则

1. **开闭原则**：对扩展开放、对修改关闭。
2. **依赖倒置**：高层与低层都依赖抽象，抽象不依赖具体。
3. **单一职责**：一个类只负责一件事。
4. **接口隔离**：接口粒度尽量小，只依赖需要的接口。
5. **迪米特法则（最少知识）**：只与直接朋友通信，降低耦合。
6. **里氏替换**：子类可替换父类且不改变父类语义。
7. **合成复用**：优先用组合/聚合而非继承实现复用。

### 10.2 创建型（5）

- **工厂方法**：定义创建接口，子类决定实例化哪个类；解耦、符合开闭。
- **抽象工厂**：创建产品族（多个产品），扩展新产品族困难。
- **单例**：全局唯一实例。写法：饿汉、懒汉（双重检查 `volatile`）、静态内部类、枚举（最安全，防反射/序列化破坏）。
- **原型**：`clone` 复制对象，注意浅拷贝/深拷贝。
- **建造者**：分步构建复杂对象（如 `StringBuilder`、`Lombok @Builder`）。

### 10.3 结构型（7）

- **代理**：为对象提供代理控制访问（静态/动态代理）。
- **门面/外观**：为子系统提供统一入口。
- **装饰器**：动态给对象添加职责（Java IO 流）。
- **享元**：共享细粒度对象减少内存（如字符串常量池、对象池）。
- **组合**：树形结构表示「整体-部分」。
- **适配器**：将一个接口转换成客户端期望的另一接口。
- **桥接**：分离抽象与实现，使二者可独立变化（多维度变化）。

### 10.4 行为型（11）

- **模板方法**：父类定义流程骨架，子类实现具体步骤。
- **策略**：算法可互换、运行时切换。
- **责任链**：请求沿链传递，直到被处理（如过滤器、审批流）。
- **迭代器**：顺序访问聚合对象而不暴露内部结构。
- **命令**：把请求封装成对象，支持撤销/排队。
- **状态**：对象内部状态改变时改变其行为。
- **观察者**：一对多依赖，状态变化通知订阅者。
- **中介者**：用中介对象封装对象间交互，降低耦合。
- **备忘录**：保存/恢复对象状态（快照）。
- **访问者**：在不改变元素类的前提下定义作用于元素的新操作。
- **解释器**：定义语言的文法并解释执行。

- 【补充】易混对比：代理 vs 装饰器（代理控制访问、装饰增强功能）；策略 vs 状态（策略由外部选择、状态由内部流转）；工厂方法 vs 抽象工厂（单一产品 vs 产品族）。

---

## 十一、数据结构与算法

### 11.1 基础数据结构

- 线性表（数组/链表）、栈（LIFO）、队列（FIFO）、串、树（二叉树、BST、AVL、红黑树、B/B+树、堆、哈夫曼树）、图（DFS/BFS、最短路径、最小生成树）、查找（顺序/二分/哈希/树表）、排序。

### 11.2 常见排序对比【补充】

| 算法 | 平均时间 | 最坏 | 空间 | 稳定 |
|---|---|---|---|---|
| 冒泡 | O(n²) | O(n²) | O(1) | 是 |
| 选择 | O(n²) | O(n²) | O(1) | 否 |
| 插入 | O(n²) | O(n²) | O(1) | 是 |
| 归并 | O(nlogn) | O(nlogn) | O(n) | 是 |
| 快排 | O(nlogn) | O(n²) | O(logn) | 否 |
| 堆排序 | O(nlogn) | O(nlogn) | O(1) | 否 |
| 计数/桶/基数 | O(n+k) | — | O(n+k) | 视实现 |

### 11.3 树与查找【补充】

- 二叉树遍历：前/中/后序（栈/递归）、层序（队列）。
- 二叉搜索树 BST：左<根<右，退化成链表时 O(n)。
- 平衡二叉树 AVL：任意节点左右子树高度差 ≤1，查找 O(logN)。
- 红黑树：近似平衡，插入/删除/查找 O(logN)（HashMap/TreeMap 底层）。
- B/B+树：多路平衡查找树，磁盘/数据库索引常用（B+ 数据全在叶子、叶子链表相连）。
- 堆：完全二叉树，大顶堆/小顶堆，可用于优先队列与 TopK。

- 【补充】算法思想：分治、贪心、动态规划（背包、最长公共子序列）、回溯、双指针、滑动窗口、位运算；常见结构：并查集、前缀和、单调栈、Trie。

---

## 十二、分布式、微服务与面试专题

> 说明：为避免重复，**Spring Boot 启动与自动配置**（见 4.6）、**线程池参数**（见 3.8）、**synchronized 锁升级**（见 3.5）等高频考点已在前文详述，本章只保留分布式、微服务等专题内容。

### 12.1 分布式事务

- **CAP**：一致性、可用性、分区容错性三者不可同时满足（P 必选，故在 C 与 A 间取舍）。**BASE**：基本可用、软状态、最终一致性。
- **2PC**：准备 + 提交两阶段，协调者控制；同步阻塞、单点、数据不一致风险。**3PC**：增加预提交阶段、引入超时，降低阻塞但仍有问题。
- **TCC**：Try-Confirm-Cancel，业务层实现补偿，适合强一致/高性能场景，侵入大。
- **Saga**：长事务拆分为本地事务序列，失败执行补偿，最终一致。
- **本地消息表 / MQ 最终一致性**：本地事务与消息表同库保证原子，异步投递 + 重试，实现最终一致。
- **Seata**：AT（自动补偿，基于 undo log）、TCC、Saga、XA 模式。
- 数据库层 **XA** 也可用于分布式事务。

### 12.2 微服务与高可用

- 组件：注册中心（Nacos/Eureka）、配置中心（Nacos/Config）、网关（Gateway/Zuul）、负载均衡（Ribbon/LoadBalancer）、远程调用（Feign/OpenFeign）、熔断限流（Sentinel/Hystrix）。
- 服务雪崩：一个下游故障导致上游线程耗尽级联失败。应对：**超时、重试（谨慎）、熔断、降级、限流、隔离（线程池/信号量）**。
- 限流算法：计数器（固定窗口）、滑动窗口、漏桶、令牌桶。
- Feign vs OpenFeign：OpenFeign 是 Spring Cloud 对 Feign 的增强（支持 SpringMVC 注解）。

### 12.3 消息队列（RabbitMQ 要点）

- 消息顺序：单队列单消费者 / 按业务 key 分区保序。
- 重复消费：业务幂等（唯一 id + 去重表/状态机）。
- 消息丢失：生产确认（publisher confirm）、Broker 持久化、消费手动 ack。
- 消息积压：扩容消费者、临时转存、优化消费逻辑。
- 死信队列（DLX）、延迟队列（TTL + DLX 或延迟插件）、高可用（镜像队列/仲裁队列）、事务（channel.txSelect）。

### 12.4 DDD 领域驱动设计

- 贫血模型 vs 充血模型：贫血只有数据+getter/setter，充血把业务行为放入领域对象。
- 核心概念：实体（Entity，有唯一标识）、值对象（Value Object，不可变）、聚合（Aggregate，聚合根保证一致性边界）、工厂（Factory）、仓储（Repository）、领域服务（Domain Service）、领域事件（Domain Event）、限界上下文（Bounded Context）。
- 架构：六边形架构（端口与适配器）、分层架构、CQRS、事件驱动。
- 【补充】DDD 适用复杂业务域；简单 CRUD 系统不必强行套用。

### 12.5 分布式数据库与分库分表【补充】

- **ShardingSphere-JDBC** 的局限：分片键依赖严重（非分片键查询差）、跨库 JOIN 几乎不可用、分布式事务复杂、扩容非弹性、深分页性能差、对应用有侵入性、多语言支持受限。
- **TiDB**：计算存储分离、PB 级、Raft 强一致、高度兼容 MySQL、HTAP；缺点：运维复杂、硬件要求高、优化器对复杂查询有局限。
- **OceanBase**：极致扩展性、金融级高可用（Paxos 三地五中心）、高压缩（成本约 MySQL 的 1/4~1/6）、单机分布式一体化（4.0 支持单机模式）。
- **分库分表**：垂直拆分（按业务/字段）与水平拆分（按行，如取模/范围/一致性哈希）；带来的问题：跨库 JOIN、分布式事务、深分页、全局唯一 ID、弹性扩容、数据迁移与聚合统计。
- **全局 ID 方案**：雪花算法（Snowflake）、号段模式（号段表）、Redis INCR、UUID（无序不推荐做主键）。

### 12.6 其他面试点

- 过滤器（Filter，Servlet 层）→ 拦截器（Interceptor，Spring MVC）→ AOP（方法级）执行顺序与区别。
- Tomcat 调优：线程池、连接数、NIO 端点、JVM 参数。
- 接口幂等、分布式 id（雪花算法/号段/Redis INCR）、分布式锁。

---

## 十三、Go 面试相关（兼收）

> 原「java（兼 go）面试题」图以 Java 为主并兼顾 Go，此处补充 Go 常考对比要点。

- **并发模型**：Go 原生支持 `goroutine`（轻量级协程，初始栈 ~2KB 可增长）+ `channel`（CSP 通信），Java 用线程 + 共享内存 + 锁/JUC。
- **GMP 调度**：G（goroutine）由 P（逻辑处理器）调度到 M（OS 线程）执行，配合工作窃取。
- **sync 包**：`sync.Mutex/RWMutex`、`WaitGroup`、`Once`、`Pool`；并发安全用 `sync.Map`（读多写少）。
- **`defer`**：函数返回前执行，常用于释放资源；注意参数在 defer 时求值。
- **接口**：隐式实现（duck typing），空接口 `interface{}` 类似 Object。
- **slice/map**：slice 是数组视图（ptr+len+cap），扩容会生成新底层数组；map 非并发安全。
- **GC**：Go 采用并发三色标记 + 混合写屏障，低延迟；Java 有分代与多种收集器。
- **错误处理**：Go 用返回值 `error` 显式处理，Java 用异常。

---

## 十四、附录：本次整理的主要修正与补充

### 修正（原图错误/过时）
1. **JDK8 默认收集器**：原文写「Parallel Scavenge + Serial Old」，更正为 **Parallel Scavenge + Parallel Old**（Server 模式）。
2. **方法区演进**：字符串常量池在 **JDK7 已移入堆**，JDK8 移除永久代改为**元空间**；原文把两者都归到 JDK8。
3. **循环依赖**：三级缓存为 `singletonObjects`/`earlySingletonObjects`/`singletonFactories`；仅能解决**单例 + Setter/字段注入**，构造器注入无法解决。
4. **Spring Boot 自动配置加载文件**：2.7 前 `spring.factories`，2.7+/3.0 改为 `META-INF/spring/...AutoConfiguration.imports`。
5. **TCP 拥塞控制**：明确快重传/快恢复触发条件与 cwnd/ssthresh 变化。
6. **Redis 单线程**：指命令执行单线程，网络 IO 在 6.0 起可多线程。
7. **synchronized 偏向锁**：JDK15 废弃、JDK18 默认禁用。
8. **HashMap/ConcurrentHashMap**：JDK8 HashMap 引入红黑树、尾插；ConcurrentHashMap JDK8 改为 CAS + synchronized 桶锁（取消 Segment）。

### 补充（常见考点）
- HashMap 树化阈值、扩容、线程不安全；ConcurrentHashMap 锁粒度。
- JMM happens-before、volatile 语义、ThreadLocal 内存泄漏与 remove。
- AQS state/CLH/Condition、ReentrantReadWriteLock 高低位、StampedLock。
- 阻塞队列类型与线程池参数/拒绝策略、Executors 弊端。
- 索引失效场景、覆盖索引、ICP、explain 关键列、深分页优化。
- InnoDB 锁类型（记录/间隙/临键）、MVCC、binlog/redo/undo 分工。
- Redis 三大缓存问题、持久化策略、集群与淘汰策略、一致性方案。
- Netty 粘包/拆包、空轮询 Bug、零拷贝、epoll 对比。
- 设计模式易混对比、单例安全写法。
- 算法复杂度对比、常见数据结构特点。

---

## 附录 A：后端研发工程师笔试题（原文件收录）

> 以下为原 `文章\后端研发工程师笔试题.docx` 的题目，作为练习保留（原题未提供标准答案）。


后端研发工程师笔试题
一、单选题

1. 以下关于JVM内存模型及垃圾回收的描述，错误的是（）
A. 堆内存分为年轻代（Eden+Survivor）和老年代，年轻代默认采用复制算法，老年代采用标记-清除或标记-整理算法
B. 方法区（元空间）存储类信息、常量、静态变量，JDK8后元空间替代永久代，直接使用本地内存
C. GC Roots包括虚拟机栈中引用的对象、方法区中类静态属性引用的对象、本地方法栈中JNI引用的对象
D. 垃圾回收器中，G1收集器适用于大堆内存，可实现停顿时间可控，而ZGC、Shenandoah收集器不支持并发回收

2. 关于Java并发编程，下列说法错误的是（）
A. synchronized关键字可修饰方法、代码块，底层通过监视器锁（monitor）实现，JDK1.8后优化了偏向锁、轻量级锁、重量级锁的升级机制
B. volatile关键字可保证可见性和禁止指令重排序，但无法保证原子性，适合作为状态标记位使用
C. ThreadLocal用于线程本地存储，每个线程持有独立的副本，可避免线程安全问题，但需注意内存泄漏（需remove()释放）
D. 线程池参数中，corePoolSize是最大线程数，maximumPoolSize是核心线程数，空闲线程超过keepAliveTime会被回收

3. 分布式系统中，关于CAP理论和BASE理论的描述，正确的是（）
A. CAP理论中，一致性（C）、可用性（A）、分区容错性（P）三者可同时满足
B. BASE理论是CAP理论的延伸，强调最终一致性，牺牲强一致性换取高可用性，适用于分布式业务场景
C. 分布式事务的2PC（两阶段提交）协议，可完全保证一致性，无任何弊端
D. 最终一致性场景中，无需任何补偿机制，数据会自动同步一致

4. 关于Spring Cloud微服务架构，下列说法错误的是（）
A. Nacos可同时实现服务注册发现和配置中心，替代Eureka+Config的组合
B. Feign基于Ribbon实现负载均衡，支持声明式接口调用，底层采用HTTP协议
C. Sentinel用于服务熔断、限流、降级，可替代Hystrix，支持动态规则配置
D. Gateway是基于Servlet实现的网关，可实现路由转发、权限校验、限流等功能，性能优于Zuul

5. MySQL数据库高级优化，下列操作错误的是（）
A. 对于频繁查询的字段，建立B+树索引；对于模糊查询（like '%xxx'），索引无法生效
B. 分库分表可解决单库单表数据量过大的问题，水平分表适用于数据量激增，垂直分表适用于字段过多
C. 事务隔离级别中，Repeatable Read（可重复读）是MySQL默认级别，可避免脏读、不可重复读、幻读
D. 慢查询优化可通过explain分析执行计划，优化SQL语句、调整索引，避免全表扫描

6. 关于Redis高级特性，下列描述错误的是（）
A. Redis支持String、Hash、List、Set、Sorted Set等数据结构，Sorted Set可用于排行榜场景
B. Redis持久化机制包括RDB和AOF，RDB适合备份，AOF适合数据恢复，可同时开启两种机制
C. Redis集群（Redis Cluster）采用主从复制+哨兵模式，可实现高可用，每个节点负责一部分哈希槽
D. Redis的缓存穿透可通过布隆过滤器解决，缓存击穿可通过互斥锁或热点数据永不过期解决

7. 下列关于设计模式的应用，错误的是（）
A. Spring中的BeanFactory采用工厂模式，负责Bean的创建和管理
B. Spring MVC中的DispatcherServlet采用观察者模式，分发请求到对应Controller
C. 单例模式可通过双重检查锁（DCL）实现线程安全，避免反射和序列化破坏单例
D. 策略模式用于定义一系列算法，可动态切换算法，如Spring中的Resource接口实现

8. 关于分布式缓存设计，下列说法错误的是（）
A. 缓存更新策略中，Cache-Aside（旁路缓存）模式适合读多写少场景，先更数据库再删缓存
B. 缓存雪崩是指大量缓存同时过期，导致请求全部打向数据库，可通过设置过期时间随机化解决
C. 本地缓存（如Caffeine）比分布式缓存（如Redis）速度快，可用于缓存热点数据，减少网络开销
D. 缓存一致性问题可通过分布式事务完全解决，无需考虑并发更新场景

9. 关于Java性能优化，下列操作不合理的是（）
A. 避免频繁创建对象，可通过对象池（如Apache Commons Pool）复用对象，减少GC压力
B. 对于大量字符串拼接，使用StringBuilder替代String，避免产生过多临时对象
C. 尽量使用ArrayList替代LinkedList，因为ArrayList的查询和修改效率都优于LinkedList
D. 合理设置JVM参数（堆内存、元空间、垃圾回收器），根据业务场景优化GC停顿时间

10. 关于微服务架构下的问题排查，下列说法错误的是（）
A. 可通过SkyWalking、Pinpoint等链路追踪工具，定位跨服务调用的异常和性能瓶颈
B. 日志收集可使用ELK（Elasticsearch+Logstash+Kibana），实现日志集中管理和检索
C. 服务监控可使用Prometheus+Grafana，监控服务指标（CPU、内存、接口响应时间），设置告警
D. 跨服务调用异常，只需排查调用方代码，无需关注被调用方的状态和日志
