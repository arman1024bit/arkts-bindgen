use std::boxed::Box;
use std::cell::{Cell, RefCell};
use std::collections::{BTreeMap, BTreeSet, BinaryHeap, HashMap, HashSet, LinkedList, VecDeque};
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};

/// 一个包含Rust中所有主要数据结构的结构体
#[allow(dead_code)]
struct AllRustDataStructures {
    // ========== 基本标量类型 ==========
    /// 有符号整数类型
    int8: i8,
    int16: i16,
    int32: i32,
    int64: i64,
    int128: i128,
    int_size: isize, // 根据平台确定大小

    /// 无符号整数类型
    uint8: u8,
    uint16: u16,
    uint32: u32,
    uint64: u64,
    uint128: u128,
    uint_size: usize, // 根据平台确定大小

    /// 浮点数类型
    float32: f32,
    float64: f64,

    /// 布尔类型
    boolean: bool,

    /// 字符类型（Unicode标量值）
    character: char,

    // ========== 复合类型 ==========
    /// 元组类型
    tuple: (i32, f64, bool, &'static str),

    /// 数组类型（固定大小）
    array: [i32; 5],

    /// 单元类型
    unit: (),

    // ========== 字符串类型 ==========
    /// 可增长的字符串（堆分配）
    string: String,

    /// 字符串切片（引用）
    str_slice: &'static str,

    // ========== 动态大小的集合类型 ==========
    /// 动态数组（向量）
    vector: Vec<i32>,

    /// 双端队列
    vec_deque: VecDeque<i32>,

    /// 链表
    linked_list: LinkedList<i32>,

    // ========== 哈希集合类型 ==========
    /// 哈希映射（键值对）
    hash_map: HashMap<String, i32>,

    /// 哈希集合（唯一值）
    hash_set: HashSet<String>,

    // ========== 有序集合类型（基于B树） ==========
    /// B树映射（有序键值对）
    btree_map: BTreeMap<i32, String>,

    /// B树集合（有序唯一值）
    btree_set: BTreeSet<i32>,

    /// 二叉堆（优先队列）
    binary_heap: BinaryHeap<i32>,

    // ========== 智能指针类型 ==========
    /// Box指针（堆分配，独占所有权）
    boxed: Box<i32>,

    /// 引用计数指针（单线程共享所有权）
    rc_ptr: Rc<String>,

    /// 原子引用计数指针（多线程共享所有权）
    arc_ptr: Arc<String>,

    /// 带内部可变性的Cell（Copy类型）
    cell: Cell<i32>,

    /// 带内部可变性的RefCell（运行时借用检查）
    ref_cell: RefCell<String>,

    // ========== 并发类型 ==========
    /// 互斥锁（独占访问）
    mutex: Mutex<i32>,

    /// 读写锁（多读单写）
    rwlock: RwLock<String>,

    // ========== Option和Result类型 ==========
    /// Option类型（可能为空的值）
    option_some: Option<i32>,
    option_none: Option<String>,

    /// Result类型（成功或错误）
    result_ok: Result<i32, String>,
    result_err: Result<String, &'static str>,

    // ========== 引用类型 ==========
    /// 不可变引用
    immutable_ref: &'static i32,

    // 注意：可变引用不能在结构体中长期存储，因为生命周期限制
    // mutable_ref: &'mut i32,  // 这通常不实际

    // ========== 原始指针类型 ==========
    /// 常量原始指针（不安全）
    const_raw_ptr: *const i32,

    /// 可变原始指针（不安全）
    mut_raw_ptr: *mut i32,

    // ========== 函数指针类型 ==========
    /// 函数指针
    fn_ptr: fn(i32, i32) -> i32,

    // ========== 闭包类型（通过Box存储） ==========
    /// 闭包（通过trait对象）
    closure: Box<dyn Fn(i32) -> i32>,

    // ========== 切片类型 ==========
    /// 切片（对数组或向量的引用视图）
    slice: &'static [i32],
}

impl AllRustDataStructures {
    /// 创建一个包含所有数据结构示例的实例
    fn new() -> Self {
        // 静态数据用于引用
        static STATIC_INT: i32 = 42;
        static STATIC_SLICE: &[i32] = &[1, 2, 3, 4, 5];

        // 创建一些共享的Rc和Arc
        let shared_rc = Rc::new(String::from("Rc共享数据"));
        let shared_arc = Arc::new(String::from("Arc共享数据"));

        AllRustDataStructures {
            // 基本标量类型
            int8: -128,
            int16: -32768,
            int32: -2147483648,
            int64: -9223372036854775808,
            int128: -170141183460469231731687303715884105728,
            int_size: -1,
            uint8: 255,
            uint16: 65535,
            uint32: 4294967295,
            uint64: 18446744073709551615,
            uint128: 340282366920938463463374607431768211455,
            uint_size: 1000,
            float32: 3.14159,
            float64: 2.718281828459045,
            boolean: true,
            character: '🦀',

            // 复合类型
            tuple: (42, 3.14, true, "元组"),
            array: [1, 2, 3, 4, 5],
            unit: (),

            // 字符串类型
            string: String::from("可增长的字符串"),
            str_slice: "字符串字面量",

            // 动态集合
            vector: vec![1, 2, 3, 4, 5],
            vec_deque: VecDeque::from([10, 20, 30]),
            linked_list: LinkedList::from([100, 200, 300]),

            // 哈希集合
            hash_map: {
                let mut map = HashMap::new();
                map.insert(String::from("键1"), 100);
                map.insert(String::from("键2"), 200);
                map
            },
            hash_set: {
                let mut set = HashSet::new();
                set.insert(String::from("元素1"));
                set.insert(String::from("元素2"));
                set
            },

            // 有序集合
            btree_map: {
                let mut map = BTreeMap::new();
                map.insert(1, String::from("第一"));
                map.insert(2, String::from("第二"));
                map
            },
            btree_set: {
                let mut set = BTreeSet::new();
                set.insert(10);
                set.insert(20);
                set
            },
            binary_heap: BinaryHeap::from([3, 1, 4, 1, 5, 9, 2, 6]),

            // 智能指针
            boxed: Box::new(999),
            rc_ptr: Rc::clone(&shared_rc),
            arc_ptr: Arc::clone(&shared_arc),
            cell: Cell::new(42),
            ref_cell: RefCell::new(String::from("RefCell内容")),

            // 并发类型
            mutex: Mutex::new(100),
            rwlock: RwLock::new(String::from("RwLock内容")),

            // Option和Result
            option_some: Some(42),
            option_none: None,
            result_ok: Ok(100),
            result_err: Err("错误信息"),

            // 引用类型
            immutable_ref: &STATIC_INT,

            // 原始指针
            const_raw_ptr: &STATIC_INT as *const i32,
            mut_raw_ptr: std::ptr::null_mut(),

            // 函数指针
            fn_ptr: add_function,

            // 闭包
            closure: Box::new(|x| x * 2),

            // 切片
            slice: STATIC_SLICE,
        }
    }

    /// 从所有数据结构参数创建实例
    /// 这个函数接受Rust中所有主要数据结构作为参数，并返回AllRustDataStructures实例
    #[allow(clippy::too_many_arguments)]
    fn from_all_types(
        // 基本标量类型
        int8: i8,
        int16: i16,
        int32: i32,
        int64: i64,
        int128: i128,
        int_size: isize,
        uint8: u8,
        uint16: u16,
        uint32: u32,
        uint64: u64,
        uint128: u128,
        uint_size: usize,
        float32: f32,
        float64: f64,
        boolean: bool,
        character: char,
        // 复合类型
        tuple: (i32, f64, bool, &'static str),
        array: [i32; 5],
        unit: (),
        // 字符串类型
        string: String,
        str_slice: &'static str,
        // 动态集合
        vector: Vec<i32>,
        vec_deque: VecDeque<i32>,
        linked_list: LinkedList<i32>,
        // 哈希集合
        hash_map: HashMap<String, i32>,
        hash_set: HashSet<String>,
        // 有序集合
        btree_map: BTreeMap<i32, String>,
        btree_set: BTreeSet<i32>,
        binary_heap: BinaryHeap<i32>,
        // 智能指针
        boxed: Box<i32>,
        rc_ptr: Rc<String>,
        arc_ptr: Arc<String>,
        cell: Cell<i32>,
        ref_cell: RefCell<String>,
        // 并发类型
        mutex: Mutex<i32>,
        rwlock: RwLock<String>,
        // Option和Result
        option_some: Option<i32>,
        option_none: Option<String>,
        result_ok: Result<i32, String>,
        result_err: Result<String, &'static str>,
        // 引用类型
        immutable_ref: &'static i32,
        // 原始指针
        const_raw_ptr: *const i32,
        mut_raw_ptr: *mut i32,
        // 函数指针
        fn_ptr: fn(i32, i32) -> i32,
        // 闭包
        closure: Box<dyn Fn(i32) -> i32>,
        // 切片
        slice: &'static [i32],
    ) -> Self {
        AllRustDataStructures {
            int8,
            int16,
            int32,
            int64,
            int128,
            int_size,
            uint8,
            uint16,
            uint32,
            uint64,
            uint128,
            uint_size,
            float32,
            float64,
            boolean,
            character,
            tuple,
            array,
            unit,
            string,
            str_slice,
            vector,
            vec_deque,
            linked_list,
            hash_map,
            hash_set,
            btree_map,
            btree_set,
            binary_heap,
            boxed,
            rc_ptr,
            arc_ptr,
            cell,
            ref_cell,
            mutex,
            rwlock,
            option_some,
            option_none,
            result_ok,
            result_err,
            immutable_ref,
            const_raw_ptr,
            mut_raw_ptr,
            fn_ptr,
            closure,
            slice,
        }
    }

    /// 演示如何使用这些数据结构
    fn demonstrate(&self) {
        println!("========== 基本类型演示 ==========");
        println!("i32: {}", self.int32);
        println!("f64: {}", self.float64);
        println!("bool: {}", self.boolean);
        println!("char: {}", self.character);

        println!("\n========== 字符串演示 ==========");
        println!("String: {}", self.string);
        println!("&str: {}", self.str_slice);

        println!("\n========== 集合演示 ==========");
        println!("Vec: {:?}", self.vector);
        println!("HashMap: {:?}", self.hash_map);
        println!("HashSet: {:?}", self.hash_set);

        println!("\n========== 智能指针演示 ==========");
        println!("Box: {}", self.boxed);
        println!(
            "Rc (强引用计数: {}): {}",
            Rc::strong_count(&self.rc_ptr),
            self.rc_ptr
        );
        println!(
            "Arc (强引用计数: {}): {}",
            Arc::strong_count(&self.arc_ptr),
            self.arc_ptr
        );

        println!("\n========== Option和Result演示 ==========");
        println!("Option Some: {:?}", self.option_some);
        println!("Option None: {:?}", self.option_none);
        println!("Result Ok: {:?}", self.result_ok);
        println!("Result Err: {:?}", self.result_err);

        println!("\n========== 函数指针和闭包演示 ==========");
        println!(
            "函数指针调用: {}(10, 20) = {}",
            "add_function",
            (self.fn_ptr)(10, 20)
        );
        println!("闭包调用: closure(5) = {}", (self.closure)(5));

        println!("\n========== 数组和切片演示 ==========");
        println!("数组: {:?}", self.array);
        println!("切片: {:?}", self.slice);
    }
}

/// 用于函数指针示例的函数
fn add_function(a: i32, b: i32) -> i32 {
    a + b
}

/// 顶层函数：接受所有Rust数据结构作为参数，返回AllRustDataStructures实例
/// 这个函数演示了如何将所有主要的Rust数据类型作为函数参数
#[allow(clippy::too_many_arguments)]
fn create_from_all_rust_types(
    // 基本标量类型
    int8: i8,
    int16: i16,
    int32: i32,
    int64: i64,
    int128: i128,
    int_size: isize,
    uint8: u8,
    uint16: u16,
    uint32: u32,
    uint64: u64,
    uint128: u128,
    uint_size: usize,
    float32: f32,
    float64: f64,
    boolean: bool,
    character: char,
    // 复合类型
    tuple: (i32, f64, bool, &'static str),
    array: [i32; 5],
    unit: (),
    // 字符串类型
    string: String,
    str_slice: &'static str,
    // 动态集合
    vector: Vec<i32>,
    vec_deque: VecDeque<i32>,
    linked_list: LinkedList<i32>,
    // 哈希集合
    hash_map: HashMap<String, i32>,
    hash_set: HashSet<String>,
    // 有序集合
    btree_map: BTreeMap<i32, String>,
    btree_set: BTreeSet<i32>,
    binary_heap: BinaryHeap<i32>,
    // 智能指针
    boxed: Box<i32>,
    rc_ptr: Rc<String>,
    arc_ptr: Arc<String>,
    cell: Cell<i32>,
    ref_cell: RefCell<String>,
    // 并发类型
    mutex: Mutex<i32>,
    rwlock: RwLock<String>,
    // Option和Result
    option_some: Option<i32>,
    option_none: Option<String>,
    result_ok: Result<i32, String>,
    result_err: Result<String, &'static str>,
    // 引用类型
    immutable_ref: &'static i32,
    // 原始指针
    const_raw_ptr: *const i32,
    mut_raw_ptr: *mut i32,
    // 函数指针
    fn_ptr: fn(i32, i32) -> i32,
    // 闭包
    closure: Box<dyn Fn(i32) -> i32>,
    // 切片
    slice: &'static [i32],
) -> AllRustDataStructures {
    // 直接调用结构体的 from_all_types 方法
    AllRustDataStructures::from_all_types(
        int8,
        int16,
        int32,
        int64,
        int128,
        int_size,
        uint8,
        uint16,
        uint32,
        uint64,
        uint128,
        uint_size,
        float32,
        float64,
        boolean,
        character,
        tuple,
        array,
        unit,
        string,
        str_slice,
        vector,
        vec_deque,
        linked_list,
        hash_map,
        hash_set,
        btree_map,
        btree_set,
        binary_heap,
        boxed,
        rc_ptr,
        arc_ptr,
        cell,
        ref_cell,
        mutex,
        rwlock,
        option_some,
        option_none,
        result_ok,
        result_err,
        immutable_ref,
        const_raw_ptr,
        mut_raw_ptr,
        fn_ptr,
        closure,
        slice,
    )
}

fn test() {
    let separator = "=".repeat(50);

    println!("Rust 所有数据结构示例\n");
    println!("{}", separator);

    // 方法1: 使用 new() 创建实例
    println!("\n【方法1】使用 new() 方法创建实例:");
    let all_structures = AllRustDataStructures::new();
    all_structures.demonstrate();

    println!("\n{}", separator);

    // 方法2: 使用 from_all_types() 从参数创建实例
    println!("\n【方法2】使用 from_all_types() 从所有数据结构参数创建实例:");

    // 准备静态数据
    static STATIC_INT: i32 = 100;
    static STATIC_SLICE: &[i32] = &[10, 20, 30];

    // 准备各种数据结构
    let my_string = String::from("自定义字符串");
    let my_vector = vec![5, 10, 15, 20];
    let my_hashmap = {
        let mut map = HashMap::new();
        map.insert(String::from("Rust"), 2015);
        map.insert(String::from("Go"), 2009);
        map
    };
    let my_rc = Rc::new(String::from("自定义Rc"));
    let my_arc = Arc::new(String::from("自定义Arc"));
    let my_closure = Box::new(|x: i32| x * 3);

    // 调用 from_all_types 函数
    let custom_structures = AllRustDataStructures::from_all_types(
        // 基本标量类型
        127,                                     // int8
        32767,                                   // int16
        2147483647,                              // int32
        9223372036854775807,                     // int64
        170141183460469231731687303715884105727, // int128
        999,                                     // int_size
        255,                                     // uint8
        65535,                                   // uint16
        4294967295,                              // uint32
        18446744073709551615,                    // uint64
        340282366920938463463374607431768211455, // uint128
        2024,                                    // uint_size
        2.71828,                                 // float32
        3.141592653589793,                       // float64
        false,                                   // boolean
        '中',                                    // character
        // 复合类型
        (100, 2.5, false, "自定义元组"), // tuple
        [10, 20, 30, 40, 50],            // array
        (),                              // unit
        // 字符串类型
        my_string,    // string
        "静态字符串", // str_slice
        // 动态集合
        my_vector,                       // vector
        VecDeque::from([100, 200, 300]), // vec_deque
        LinkedList::from([1000, 2000]),  // linked_list
        // 哈希集合
        my_hashmap, // hash_map
        {
            let mut set = HashSet::new();
            set.insert(String::from("Rust"));
            set.insert(String::from("Python"));
            set
        }, // hash_set
        // 有序集合
        {
            let mut map = BTreeMap::new();
            map.insert(1, String::from("一"));
            map.insert(2, String::from("二"));
            map.insert(3, String::from("三"));
            map
        }, // btree_map
        BTreeSet::from([1, 2, 3, 5, 8, 13]), // btree_set
        BinaryHeap::from([42, 17, 91, 8]),   // binary_heap
        // 智能指针
        Box::new(12345),                             // boxed
        my_rc,                                       // rc_ptr
        my_arc,                                      // arc_ptr
        Cell::new(888),                              // cell
        RefCell::new(String::from("自定义RefCell")), // ref_cell
        // 并发类型
        Mutex::new(777),                           // mutex
        RwLock::new(String::from("自定义RwLock")), // rwlock
        // Option和Result
        Some(999),         // option_some
        None,              // option_none
        Ok(200),           // result_ok
        Err("自定义错误"), // result_err
        // 引用类型
        &STATIC_INT, // immutable_ref
        // 原始指针
        &STATIC_INT as *const i32, // const_raw_ptr
        std::ptr::null_mut(),      // mut_raw_ptr
        // 函数指针
        add_function, // fn_ptr
        // 闭包
        my_closure, // closure
        // 切片
        STATIC_SLICE, // slice
    );

    println!("\n使用 from_all_types() 创建的实例演示:");
    custom_structures.demonstrate();

    println!("\n{}", separator);

    // 方法3: 使用顶层函数 create_from_all_rust_types()
    println!("\n【方法3】使用顶层函数 create_from_all_rust_types():");

    // 准备更多数据
    static ANOTHER_INT: i32 = 42;
    static ANOTHER_SLICE: &[i32] = &[7, 14, 21, 28];

    let third_instance = create_from_all_rust_types(
        // 基本标量类型
        -100,
        -1000,
        -10000,
        -100000,
        -1000000,
        -10,
        200,
        2000,
        20000,
        200000,
        2000000,
        3000,
        1.414,
        1.732,
        true,
        '🚀',
        // 复合类型
        (999, 9.99, true, "顶层函数"),
        [100, 200, 300, 400, 500],
        (),
        // 字符串
        String::from("顶层函数创建"),
        "顶层函数字符串",
        // 动态集合
        vec![111, 222, 333],
        VecDeque::from([11, 22, 33, 44]),
        LinkedList::from([1, 1, 2, 3, 5, 8]),
        // 哈希集合
        {
            let mut map = HashMap::new();
            map.insert(String::from("Rust"), 1);
            map.insert(String::from("C++"), 2);
            map.insert(String::from("Python"), 3);
            map
        },
        {
            let mut set = HashSet::new();
            set.insert(String::from("编程"));
            set.insert(String::from("语言"));
            set
        },
        // 有序集合
        {
            let mut map = BTreeMap::new();
            map.insert(10, String::from("十"));
            map.insert(20, String::from("二十"));
            map
        },
        BTreeSet::from([2, 4, 6, 8, 10]),
        BinaryHeap::from([100, 50, 75, 25]),
        // 智能指针
        Box::new(54321),
        Rc::new(String::from("顶层Rc")),
        Arc::new(String::from("顶层Arc")),
        Cell::new(1024),
        RefCell::new(String::from("顶层RefCell")),
        // 并发类型
        Mutex::new(2048),
        RwLock::new(String::from("顶层RwLock")),
        // Option和Result
        Some(12345),
        None,
        Ok(54321),
        Err("顶层函数错误"),
        // 引用和指针
        &ANOTHER_INT,
        &ANOTHER_INT as *const i32,
        std::ptr::null_mut(),
        // 函数指针和闭包
        add_function,
        Box::new(|x: i32| x * x),
        ANOTHER_SLICE,
    );

    println!("\n使用顶层函数 create_from_all_rust_types() 创建的实例演示:");
    third_instance.demonstrate();

    println!("\n{}", separator);
    println!("\n✅ 成功展示了三种创建方式：");
    println!("   1. AllRustDataStructures::new() - 使用默认值创建");
    println!("   2. AllRustDataStructures::from_all_types() - 方法调用，从参数创建");
    println!("   3. create_from_all_rust_types() - 顶层函数，从参数创建");
    println!("\n🎯 函数签名包含了Rust语言中所有主要的数据结构类型！");
}
