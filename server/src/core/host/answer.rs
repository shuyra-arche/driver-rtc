pub enum Answer {
    /// 「现在几点」：答一个时刻
    Time(u64),
    /// 「再过多 long 叫我」收下了：那一格占上，设备要武装到 `at`；答码是 frame::OK
    Armed {
        at: u64,
    },
    Refused {
        code: u8,
        at: u64,
    },
}
