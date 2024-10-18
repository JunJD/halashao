// 基础配置
pub const PLATFORM: &str = "xhs";
pub const KEYWORDS: &str = "月嫂 合肥";
pub static mut LOGIN_TYPE: &str = "qrcode";  // qrcode or phone or cookie
pub const COOKIES: &str = "";
// 具体值参见media_platform.xxx.field下的枚举值，暂时只支持小红书
pub const SORT_TYPE: &str = "popularity_descending";
// 具体值参见media_platform.xxx.field下的枚举值，暂时只支持抖音
pub const PUBLISH_TIME_TYPE: i32 = 0;
pub const CRAWLER_TYPE: &str = "search";  // 爬取类型，search(关键词搜索) | detail(帖子详情)| creator(创作者主页数据)

// 是否开启 IP 代理
pub const ENABLE_IP_PROXY: bool = false;

// 代理IP池数量
pub const IP_PROXY_POOL_COUNT: i32 = 2;

// 代理IP提供商名称
pub const IP_PROXY_PROVIDER_NAME: &str = "kuaidaili";

// 设置为true不会打开浏览器（无头浏览器）
// 设置false会打开一个浏览器
// 小红书如果一直扫码登录不通过，打开浏览器手动过一下滑动验证码
// 抖音如果一直提示失败，打开浏览器看下是否扫码登录之后出现了手机号验证，如果出现了手动过一下再试。
pub const HEADLESS: bool = false;

// 是否保存登录状态
pub const SAVE_LOGIN_STATE: bool = false;

// 数据保存类型选项配置,支持三种类型：csv、db、json, 最好保存到DB，有排重的功能。
pub const SAVE_DATA_OPTION: &str = "csv";  // csv or db or json

// 用户浏览器缓存的浏览器文件配置
pub const USER_DATA_DIR: &str = "%s_user_data_dir";  // %s will be replaced by platform name

// 爬取开始页数 默认从第一页开始
pub const START_PAGE: i32 = 1;

// 爬取视频/帖子的数量控制
pub const CRAWLER_MAX_NOTES_COUNT: i32 = 100;

// 并发爬虫数量控制
pub const MAX_CONCURRENCY_NUM: i32 = 1;

// 是否开启爬图片模式, 默认不开启爬图片
pub const ENABLE_GET_IMAGES: bool = false;

// 是否开启爬NOTE模式
pub const FETCH_NOTE_DETAILS: bool = false;  // 设置为true表示获取note详情,false则不获取

// 是否开启爬评论模式, 默认不开启爬评论
pub const ENABLE_GET_COMMENTS: bool = true;

// 是否开启爬二级评论模式, 默认不开启爬二级评论
pub const ENABLE_GET_SUB_COMMENTS: bool = true;

// 指定小红书需要爬虫的笔记ID列表
pub const XHS_SPECIFIED_ID_LIST: &[&str] = &[
    "6422c2750000000027000d88",
    // ........................
];

// 指定小红书创作者ID列表
pub const XHS_CREATOR_ID_LIST: &[&str] = &[
    "63e36c9a000000002703502b",
    // ........................
];

// 词云相关
// 是否开启生成评论词云图
pub const ENABLE_GET_WORDCLOUD: bool = false;

// 自定义词语及其分组
// 添加规则：xx:yy 其中xx为自定义添加的词组，yy为将xx该词组分到的组名。
pub const CUSTOM_WORDS: &[(&str, &str)] = &[
    ("零几", "年份"),  // 将"零几"识别为一个整体
    ("高频词", "专业术语")  // 示例自定义词
];

// 停用(禁用)词文件路径
pub const STOP_WORDS_FILE: &str = "./docs/hit_stopwords.txt";

// 中文字体文件路径
pub const FONT_PATH: &str = "./docs/STZHONGS.TTF";
