# Java 面试题整理（精编版）

> 由 `01_Java\面试与简历` 下的 `java面试题.docx`、`java面试题(2).docx`、`java面试题(2).pdf` 合并整理而成。
> 三个文件内容基本相同（(2).docx 为编号清晰版，(2).pdf 为其导出），已去重合并；文中对原稿的笔误/错误做了修正（**【修正】**），并补充了高频要点（**【补充】**）。
> 生成日期：2026-11-01

## 目录

1. Java 基础（集合 / 异常 / 字符串）
2. JVM（运行时数据区 / 垃圾回收 / 调优）
3. 多线程、线程池与锁
4. MySQL
5. Redis
6. Spring
7. Spring Boot
8. RabbitMQ
9. DDD 领域驱动设计
10. Netty
11. 微服务与分布式
12. 附录：本次整理的修正与补充

---

## 一、Java 基础

### 1.1 集合（List / Set / Map）

- **List（有序、可重复）**
  - `ArrayList`：有序、线程不安全、可重复；内存连续（数组），**查询快、增删慢**；初始容量 10，扩容为原容量的 **1.5 倍**。
  - `LinkedList`：线程不安全、可重复；**双向链表**，增删快、随机访问慢。
- **Set（无序、不可重复）**
  - `HashSet`：基于 HashMap，无序、不重复。
  - `LinkedHashSet`：哈希表 + 双向链表，**保持插入顺序**、不重复、线程不安全。
  - `TreeSet`：红黑树，按元素大小排序（自然序/Comparator）。
- **Map（键值对）**
  - `HashMap`：线程不安全；**JDK7 数组+链表，JDK8 数组+链表+红黑树**；初始容量 16，负载因子 0.75（容量超 75% 触发扩容），**扩容为原来 2 倍**（16→32→64）。
  - `Hashtable`：`synchronized` 实现线程安全，底层数组+链表（JDK8 也引入红黑树），key/value 不允许 null。
  - `LinkedHashMap`：保持插入/访问顺序（可做 LRU）。
  - `ConcurrentHashMap`：并发安全；**JDK7 分段锁（Segment，粒度大），JDK8 改为 CAS + synchronized 锁桶头节点（粒度小）**，结构数组+链表+红黑树。

- 【补充】`ArrayList` 扩容 1.5 倍、`HashMap` 扩容 2 倍；`HashMap` 线程不安全，多线程用 `ConcurrentHashMap`。

### 1.2 异常

- **检查型异常（Checked）**：编译期必须显式处理（`try-catch` 或 `throws`），如 `IOException`、`SQLException`。
- **非检查型异常（RuntimeException）**：编译期不强制处理，运行时才可能抛出，如空指针 `NullPointerException`、数组越界 `ArrayIndexOutOfBoundsException`、算术异常 `ArithmeticException`。
- 【补充】`Error`（如 `OutOfMemoryError`、`StackOverflowError`）属严重错误，通常不应 catch；`finally` 一般必执行。

### 1.3 字符串拼接

- `String` 不可变，用 `+` 拼接会不断创建新对象。
- `StringBuilder`：**线程不安全、性能高**；`StringBuffer`：**线程安全、性能相对低**。
- 循环内拼接优先用 `StringBuilder`。

---

## 二、JVM

### 2.1 运行时数据区

- **方法区（线程共享）**：存类信息、方法信息、JIT 编译代码、静态变量、运行时常量池（含字符串常量池）。
  - 【修正】**JDK7 起字符串常量池移入堆**；**JDK8 移除永久代改为元空间 Metaspace（本地内存）**。原稿把两者都归到 JDK8，实为两次变化。
  - 方法区实现：JDK7 及以前为永久代（堆内），JDK8+ 为元空间（本地内存）。
  - 相关错误：`OutOfMemoryError: PermGen space`（JDK7-）/ `Metaspace`（JDK8+）。
- **堆（线程共享）**：唯一目的存放对象实例，是 GC 管理区域（GC 堆）。
  - 默认比例：新生代 : 老年代 = 1 : 2（新生代约占堆 1/3，老年代 2/3）。
  - 新生代 = Eden : Survivor0 : Survivor1 = **8 : 1 : 1**；新生代触发 Minor GC，老年代触发 Full GC。
  - 异常：`OutOfMemoryError: Java heap space`。
- **虚拟机栈（线程隔离）**：每个方法执行时创建栈帧（局部变量表、操作数栈、动态链接、方法返回地址）；递归过深 → `StackOverflowError`，扩展失败 → `OutOfMemoryError`。
- **本地方法栈（线程隔离）**：为 Native 方法服务，异常同上。
- **程序计数器（线程隔离）**：当前线程字节码行号指示器，**唯一不会 OOM 的区域**。

### 2.2 直接内存

- 不属于运行时数据区，也不是 JVM 规范定义区域；NIO 的 `ByteBuffer`（DirectByteBuffer）使用堆外内存，避免堆与 Native 堆间复制，提升性能。
- 可能抛出 `OutOfMemoryError: Direct buffer memory`。

### 2.3 垃圾回收与收集器

- 判断存活：可达性分析（GC Roots），非引用计数。
- 算法：标记-清除（有碎片）、复制（新生代）、标记-整理（老年代）、分代收集。
- 收集器：
  - **新生代**：Serial（单线程复制，Client/单核）、ParNew（Serial 多线程版，配 CMS）、Parallel Scavenge（多线程，追求吞吐量）。
  - **老年代**：Serial Old（单线程，标记-整理，CMS 后备）、Parallel Old（多线程，标记-整理，配 Parallel Scavenge）、CMS（最短停顿，四阶段：初始标记、并发标记、重新标记、并发清除，有碎片）。
  - **整堆**：G1（Region 化整为零，可预测停顿，JDK9 默认）、ZGC（超低停顿、大堆、着色指针+读屏障）、Shenandoah。
- 【修正】默认收集器：JDK7/8 = Parallel Scavenge + Parallel Old；JDK9+ = G1。
- 【补充】CMS 适合响应优先，但有碎片/CPU 敏感；G1 适合大内存可预测停顿；ZGC 适合极低延迟超大堆。

### 2.4 调优

- 具体问题具体分析：低延迟、高吞吐、低内存占用三者通常「三选二」。
- 内存溢出：调大 `-Xmx`/`-Xms` 或优化代码；内存泄漏：用工具（jmap/jstack/MAT/arthas）定位泄漏代码。
- 【补充】常用参数：`-Xms/-Xmx` 堆、`-Xmn` 新生代、`-Xss` 栈、`-XX:MetaspaceSize`、`-XX:+UseG1GC`、`-XX:+HeapDumpOnOutOfMemoryError`。

---

## 三、多线程、线程池与锁

### 3.1 线程基础

- 线程六状态：NEW（新建）、RUNNABLE（就绪/运行）、BLOCKED（阻塞）、WAITING（无限等待）、TIMED_WAITING（限时等待）、TERMINATED（终止）。
- 创建方式：继承 `Thread`、实现 `Runnable`、实现 `Callable`（有返回值，配合 FutureTask）、线程池。
- 【补充】`start()` 才启动新线程，直接调 `run()` 只是普通方法调用。

### 3.2 ThreadLocal

- 为每个线程提供独立变量副本，底层 `ThreadLocalMap`；key 是 ThreadLocal 弱引用，**value 是强引用**。
- 内存泄漏：线程池复用线程时不 `remove()`，value 无法回收；用完务必 `ThreadLocal.remove()`。
- 典型应用：Spring 事务管理用 ThreadLocal 存数据库连接，保证同一事务同一连接。

### 3.3 volatile / CAS / JUC / AQS

- `volatile`：①保证可见性 ②不保证原子性 ③禁止指令重排序。
- **CAS**：比较并交换（`Unsafe.compareAndSwapXxx`），乐观锁/无锁；**ABA 问题**用版本号解决（`AtomicStampedReference`）。
- **AQS**：`volatile int state` + CLH 队列 + CAS，JUC 锁与同步器的底层基础；`ReentrantLock`（可重入）、`ReentrantReadWriteLock`（读写分离）均基于 AQS。

### 3.4 线程池

- 创建方式：`Executors` 工厂（**不推荐，可能有界/无界队列 OOM 风险**）或手动 `new ThreadPoolExecutor`。
- **7 个参数**：核心线程数、最大线程数、空闲存活时间、时间单位、任务队列、线程工厂、拒绝策略。
- 执行流程：核心线程满 → 入队列 → 队列满则创建非核心线程直到最大 → 仍满触发拒绝策略。
- 任务队列：无界链表队列（`LinkedBlockingQueue`）、有界数组队列（`ArrayBlockingQueue`）、优先级无界队列（`PriorityBlockingQueue`）、延迟无界队列（`DelayQueue`）。
- 拒绝策略：`AbortPolicy`（默认抛异常）、`CallerRunsPolicy`（主线程执行）、`DiscardOldestPolicy`（丢最旧+加最新）、`DiscardPolicy`（直接丢弃）。
- 【补充】生产建议手动创建、有界队列、自定义线程名与拒绝策略；`submit` 返回 Future，异常通过 get 抛出。

### 3.5 锁

- `synchronized`：锁升级 `无锁 → 偏向锁 → 轻量级锁 → 重量级锁`；修饰实例方法锁 `this`，修饰静态方法锁 `Class` 对象，修饰代码块锁指定对象。**【修正】偏向锁 JDK15 废弃、JDK18 默认禁用。**
- `Lock`：`ReentrantLock`（可重入，可设公平锁）、`ReentrantReadWriteLock`（读锁共享、写锁排他）。
- 【补充】`synchronized` 是 JVM 关键字、自动释放、非公平；`Lock` 是 API、需手动 `unlock`、可中断/可超时/可公平。

### 3.6 并发工具类

- `CountDownLatch`（倒计数，一次性）、`CyclicBarrier`（可循环屏障）、`Semaphore`（信号量限流）、`Exchanger`（两线程交换数据）。
- 【补充】`CompletableFuture`（异步编排）、`ConcurrentHashMap`（并发 Map）、`CopyOnWriteArrayList`（读多写少）。

---

## 四、MySQL

### 4.1 存储引擎

- **InnoDB**：支持事务、行锁、外键、MVCC、崩溃恢复，MySQL 5.5+ 默认引擎。
- **MyISAM**：**不支持事务、不支持外键**，仅支持**表级锁**，支持全文索引，读快但崩溃难恢复。**【修正】原稿「MyISAM 支持外键」错误，且其不属于 RR 隔离级别讨论范畴。**

### 4.2 优化与 EXPLAIN

- 优化手段：参数优化（缓冲池、日志、连接线程、排序/索引缓冲等）、索引优化、读写分离、分库分表、冷热数据分离。
- **EXPLAIN 关键列**【修正 type 顺序】：
  - `type`：`system > const > eq_ref > ref > range > index > ALL`（至少到 range，理想 ref/eq_ref）。
  - `key`：实际使用的索引（确保命中复合索引最左前缀）。
  - `rows`：预估扫描行数（越小越好，避免全表扫描）。
  - `Extra`：`Using index`（覆盖索引，好）、`Using where`、`Using filesort`/`Using temporary`（需优化）。

### 4.3 索引

- 按列属性：单列索引、联合索引（最左前缀）。
- 按数据结构：B+Tree（数据在叶子节点、叶子链表相连，适合范围/排序）、Hash（仅等值查询）、R-Tree（空间索引）。
- 按存储结构：聚簇索引（索引+整行数据，物理顺序≈主键，一张表一个）、辅助索引（叶子存索引列+主键，查非索引列需**回表**）。
- 【补充】覆盖索引避免回表；索引失效：`like '%x'`、函数/运算、隐式类型转换、`or`、违反最左前缀等。

### 4.4 事务 ACID 与日志

- **原子性**：`undo log`（回滚日志）。
- **一致性**：由其他三个特性共同保证。
- **隔离性**：锁机制 + MVCC。
- **持久性**：`redo log`（重做日志，先写日志再改页）。
- **binlog**：Server 层逻辑日志，记录修改语句，用于主从复制与数据恢复。
- 【补充】redo log（InnoDB 物理/逻辑日志，崩溃恢复）+ binlog（Server 层，复制/归档）通过两阶段提交保证一致。

### 4.5 锁、MVCC 与隔离级别

- **MVCC**：多版本并发控制，由隐式字段（trx_id、roll_pointer）+ undo log + ReadView 实现，使读写不冲突。
- **间隙锁（Gap Lock）**：防止幻读，在可重复读（RR）下生效；InnoDB 用 Next-Key Lock（记录锁+间隙锁）。
- **记录锁**：在索引记录上加锁；**查询未走索引时可能升级为表锁**。
- 隔离级别与并发问题：
| 隔离级别 | 脏读 | 不可重复读 | 幻读 |
|---|---|---|---|
| 读未提交 | 可能 | 可能 | 可能 |
| 读已提交 | 否 | 可能 | 可能 |
| 可重复读（MySQL 默认） | 否 | 否 | InnoDB 基本避免（间隙锁/MVCC） |
| 串行化 | 否 | 否 | 否 |

### 4.6 死锁

- 查看：`SHOW ENGINE INNODB STATUS;`（含死锁 SQL 与锁状态），错误日志 `error.log`。
- InnoDB 会自动检测并回滚代价较小的事务；优化：统一资源访问顺序、减少锁持有时间、合理使用索引、降低隔离级别、使用显式锁。

---

## 五、Redis

### 5.1 数据类型

- 五种基础类型：String、List、Set、Hash、ZSet（有序集合）。
- 高级类型：Bitmaps（位图，统计登录状态）、HyperLogLog（基数统计，UV）、Geospatial（地理位置）、Pub/Sub（发布订阅）、Stream（消息流）。

### 5.2 持久化

- **RDB**：快照，文件小、恢复快、性能好，但可能丢最后一次快照后的数据。
- **AOF**：追加命令日志，实时性高、数据更安全，但文件大、恢复慢、增加 I/O。
- 【补充】生产常用 RDB + AOF 混合持久化（Redis 4.0+）。

### 5.3 缓存三大问题与一致性

- **缓存击穿**：某热点 key 过期瞬间大量请求打到 DB；方案：互斥锁/分布式锁重建、热点 key 不过期。
- **缓存穿透**：缓存和 DB 都不存在的数据；方案：布隆过滤器、缓存空值、参数校验。
- **缓存雪崩**：大量 key 同时过期或 Redis 宕机；方案：过期时间加随机值、集群高可用、限流降级。
- **缓存与数据库一致性**：先更新库再删缓存（Cache-Aside）+ 延迟双删；强一致可用分布式锁；最终一致可用 MQ/binlog（Canal）异步失效。

### 5.4 分布式锁与其他

- **Redisson**：可重入、可重试、可自动续期的分布式锁，基于 `setnx` 互斥命令实现。
- **看门狗（watch dog）**：业务未执行完时自动续期；客户端宕机则续期停止、锁到期释放。
- **Redis 事务**：支持（MULTI/EXEC），但**不支持回滚**，按顺序执行、出错继续。
- **内存淘汰策略**【补充完整】：`noeviction`、`allkeys-lru`、`volatile-lru`、`allkeys-random`、`volatile-random`、`volatile-ttl`、`allkeys-lfu`、`volatile-lfu`。
- **跳表（skiplist）**：ZSet 底层，多层链表，查找/插入/删除平均 O(logN)。
- **INCR**：可用于订单 id 自增（注意持久化与单点）。
- **内存溢出处理**：扩容、配置 maxmemory + 淘汰策略、分析并删除大 key/热 key、排查代码问题。

---

## 六、Spring

### 6.1 Bean 生命周期与作用域

- 生命周期：**实例化 → 属性赋值（依赖注入）→ 初始化（BeanPostProcessor/afterPropertiesSet/init-method）→ 使用 → 销毁**。
- 作用域：`singleton`（单例，默认）、`prototype`（原型）；Web 环境还有 `request`、`session`、`application`、`websocket`。

### 6.2 IOC 与 AOP

- **IOC（控制反转）**：对象创建与依赖关系由容器在运行时注入，而非代码硬编码；被容器管理的对象称 Bean。
- **AOP（面向切面）**：不修改源码增强功能（事务、日志、权限）。
- 代理实现**【修正】**：Spring AOP 对**实现了接口的 Bean 默认用 JDK 动态代理**（基于接口），**无接口用 CGLIB**（基于子类）；**Spring Boot 2.x 起默认 `proxyTargetClass=true`（CGLIB）**。JDK 代理创建快、调用稍慢；CGLIB 创建慢（生成子类）但调用快。

### 6.3 循环依赖

- 用**三级缓存**解决单例 Bean 的字段/Setter 循环依赖（`singletonObjects` / `earlySingletonObjects` / `singletonFactories`）；第三级工厂用于延迟生成早期引用（含 AOP 代理）。
- 【修正】**构造器注入的循环依赖无法解决**，`prototype` 也无法解决。

### 6.4 Spring MVC

- 请求流程：`DispatcherServlet` 接收请求 → `HandlerMapping` 找到处理器 → `HandlerAdapter` 调用 Controller（参数解析）→ 返回 `ModelAndView` 或 `@ResponseBody` 序列化 → `ViewResolver` 渲染视图。

### 6.5 事务隔离级别与传播行为

- 隔离级别：默认（跟随数据库）、读未提交、读已提交、可重复读、串行化。
- 传播行为：REQUIRED（默认，有则加入无则新建）、REQUIRES_NEW（挂起当前、新建）、SUPPORTS（有则加入无则普通执行）、NOT_SUPPORTED（挂起事务普通执行）、MANDATORY（必须存在，否则异常）、NEVER（存在则异常）、NESTED（嵌套事务，子回滚不影响父，父影响子）。
- 【补充】`@Transactional` 失效：自调用、非 public、异常被吞、受检异常未设 rollbackFor、引擎不支持事务。

---

## 七、Spring Boot

### 7.1 启动流程

1. 创建 `SpringApplication`：判断是否 Web 应用、从 `META-INF/spring.factories` 加载初始化器（ApplicationContextInitializer）与监听器（ApplicationListener）、找出主类。
2. 调用 `run()`：准备环境（Environment）、创建并 `refresh` `ApplicationContext`、启动内嵌 Web 服务器、执行 Runner、发布就绪事件。

### 7.2 自动配置

- `@SpringBootApplication` = `@SpringBootConfiguration` + `@EnableAutoConfiguration` + `@ComponentScan`。
- `@EnableAutoConfiguration` → `@Import(AutoConfigurationImportSelector)` → 扫描并加载自动配置类（**2.7 前 `META-INF/spring.factories`；2.7+/3.0 改为 `META-INF/spring/...AutoConfiguration.imports`**），条件注解满足时才实例化并注册到容器。

### 7.3 配置文件加载优先级

- `bootstrap.properties > bootstrap.yml > application.properties > application.yml`（bootstrap 由 Spring Cloud 提供；Spring Boot 常规为 application）。命令行参数 > 配置文件 > 默认值。

### 7.4 可执行 Jar vs 普通 Jar

- Spring Boot Jar：内嵌 Tomcat、包含依赖与配置，`java -jar` 可直接启动（`LaunchedURLClassLoader` 加载 BOOT-INF）。
- 普通 Jar：需外部容器（如 Tomcat）部署启动。

### 7.5 多数据源

- 配置文件中配置多个数据源参数 → 定义多个 `DataSource`/`SqlSessionFactory`/`TransactionManager` → 使用时用注解（如 `@DS`）指定数据源与事务管理器。

---

## 八、RabbitMQ

- **消息顺序**：单队列单消费者保序；多消费者用「单活模式（exclusive consumer）」或按业务 key 分区 + 消息携带顺序号。
- **重复消费（幂等）**：用 Redis `setnx` + 全局消息 id，能写入才消费、消费成功再 ack；或业务唯一键去重。
- **消息丢失**：生产端确认机制（publisher confirm）/事务机制；队列与消息持久化；消费端手动 ack（关闭 autoAck）。
- **消息积压**：资源不足 → 扩容消费者/增加队列；代码问题 → 回滚修复、临时转存后处理。
- **死信队列（DLX）**：消息 TTL 过期、队列达最大长度、消息被拒绝（basic.reject/nack 且 requeue=false）时进入死信队列。
- **延迟队列**：TTL + 死信交换机转发，或使用延迟插件。
- **高可用**：镜像队列/仲裁队列（经典镜像模式性能较差）；集群多节点互为副本。
- **事务机制**：基于 AMQP 的 `txSelect/txCommit/txRollback`（生产者到 Broker）。

---

## 九、DDD 领域驱动设计

- **贫血模型 vs 充血模型**：贫血只有数据 + getter/setter，行为在 Service 层（面向过程）；充血把状态与行为封装在领域对象中（面向对象）。
- **实体（Entity）**：有唯一标识（ID），包含属性与行为。
- **值对象（Value Object）**：无唯一标识、不可变（如金额、地址）。
- **聚合（Aggregate）**：聚合根作为统一对外入口，协调内部实体/值对象按业务规则保持一致，避免数据不一致。
- **工厂（Factory）**：封装复杂对象的创建，隔离变化。
- **领域服务（Domain Service）**：处理跨实体的领域逻辑（多个聚合协作）。
- **领域事件（Domain Event）**：领域内发生的重要事件（如手机号变更通知上下游）。
- **限界上下文（Bounded Context）**：业务边界，同一概念在不同上下文含义/关注点不同（如「商品」在销售与物流中关注点不同）。
- **六边形架构**：端口与适配器，依赖倒置（接口），领域层不依赖任何具体基础设施（换数据库不影响领域）。
- **领域**：业务范围，可分为核心域、支撑子域、通用子域。

---

## 十、Netty

- **IO 模型**：阻塞 IO、非阻塞 IO、IO 多路复用（select/poll/epoll）、信号驱动 IO、异步 IO。
- **BIO / NIO / AIO**：BIO 同步阻塞（一连接一线程）；NIO 同步非阻塞（多连接一线程，多路复用）；AIO 异步非阻塞（回调）。
- **select / poll / epoll**：select 轮询、有连接数限制、效率低；poll 链表无最大连接数限制、仍轮询；epoll 事件驱动、效率不随 fd 数量下降（LT/ET 模式）。
- **核心组件**：`Channel`（通道）、`EventLoop`/`EventLoopGroup`（bossGroup 处理连接、workerGroup 处理读写）、`ChannelHandler`（处理器）、`ChannelPipeline`（Handler 链表，数据依次流经）、`Selector`（多路复用选择器）、`Future/ChannelFuture`（异步结果）、`ByteBuf`（缓冲区）。
- **空轮询 Bug**：统计无 IO 事件次数，发生空轮询则重建 Selector 替换（**Netty 只是绕开，并未真正修复 JDK 的 bug**）。
- **粘包/拆包**：定长、分隔符、**长度字段**（LengthFieldBasedFrameDecoder）、自定义协议。
- **Reactor 模式**：单线程 Reactor（低并发）、多线程 Reactor（主线程 accept、线程池处理）、主从 Reactor 多线程（主 Reactor 连接、子 Reactor IO、线程池业务，超高并发）。

---

## 十一、微服务与分布式

### 11.1 微服务组件

- 网关（限流、鉴权、日志、负载均衡）、配置中心（集中管理、动态刷新）、注册中心（服务注册发现、负载均衡、故障转移）、分布式链路追踪（调用链）、限流组件、远程调用（Feign/OpenFeign）。

### 11.2 分布式事务

- **CAP → BASE**：CAP（一致性 C、可用性 A、分区容错 P 不可同时满足，P 必选）；BASE（基本可用、软状态、最终一致性，通过牺牲强一致换取可用与性能）。
- **2PC**：准备 + 提交两阶段，同步阻塞、单点、数据一致风险。
- **3PC**：在协调者与参与者都引入超时、增加 CanCommit 询问阶段，降低阻塞但仍有数据一致问题。
- **TCC**：Try（检查并预留资源）→ Confirm（使用预留资源，需幂等）→ Cancel（释放预留，需幂等）；业务侵入高、并发高。
- **Saga**：长事务拆为子事务序列，失败执行补偿；有/无中心协调者；补偿设计复杂、可能有并发冲突。
- **本地消息表**：本地事务与消息表同库保证原子，异步投递 + 重试，实现最终一致。
- **MQ**：解耦、异步、削峰；带来系统复杂性、可用性降低、一致性问题。
- **Seata**：AT（undo log 自动补偿）、TCC、Saga、XA。

### 11.3 服务治理

- **服务雪崩**：某服务性能下降导致上下游连锁故障；应对：超时、熔断、限流、降级、隔离。
- **服务降级**：压力过大时关闭非核心功能，保障核心功能。
- **服务熔断**：像保险丝，故障达阈值时中断请求，避免故障扩散（有熔断/半开/关闭状态）。
- **限流算法**：固定窗口计数器（简单，有边界突变）、滑动窗口（更平滑、较复杂）、漏桶（平滑流量、满则拒绝）、令牌桶（允许突发、控制更强）。

### 11.4 Feign / OpenFeign / Ribbon

- **Feign**：声明式 HTTP 客户端，内置 Ribbon 做负载均衡。
- **OpenFeign**：Spring Cloud 在 Feign 基础之上支持 SpringMVC 注解（`@RequestMapping` 等），通过动态代理生成实现类并完成负载均衡与调用。
- **Ribbon**：客户端负载均衡器，支持轮询、随机、重试、最可用、响应时间等策略，具备故障检测与客户端配置化。

---

## 十二、附录：本次整理的修正与补充

### 修正
1. **方法区/常量池演进**：字符串常量池 JDK7 入堆、JDK8 转元空间（原稿都归到 JDK8）。
2. **MyISAM**：不支持事务与外键、仅表锁（原稿「支持外键」错误）。
3. **EXPLAIN type 顺序**：补全为 `system>const>eq_ref>ref>range>index>ALL`（原稿遗漏 eq_ref/index）。
4. **Spring AOP 代理**：有接口默认 JDK 动态代理、无接口 CGLIB，Boot 2.x 起默认 CGLIB（原稿表述混乱）。
5. **循环依赖**：仅单例 + Setter/字段可解决，构造器注入/原型无法解决。
6. **synchronized 偏向锁**：JDK15 废弃、JDK18 默认禁用。

### 补充
- 集合扩容（ArrayList 1.5 倍 / HashMap 2 倍）、fail-fast、equals/hashCode。
- JVM 堆分代比例 8:1:1、默认收集器随 JDK 版本变化、常用调优参数。
- 线程池 7 参数与执行流程、拒绝策略、AQS、并发工具类、CompletableFuture。
- MySQL 覆盖索引/回表/索引失效、redo/undo/binlog 分工、隔离级别对照表。
- Redis 8 种淘汰策略、混合持久化、缓存一致性方案。
- Spring Bean 生命周期/作用域、AOP 代理选择、SpringMVC 流程。
- Spring Boot 自动配置加载文件版本差异、多数据源。
- Netty 粘包长度字段方案、Reactor 三种模型。
- 微服务 CAP/BASE、分布式事务各方案、限流四种算法。

---

_由 3 个同名面试题文件合并整理，生成于 2026-11-01。_
