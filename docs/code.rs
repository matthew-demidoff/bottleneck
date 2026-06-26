use std::时间::时刻;

struct 瓶颈 {
    容量: f64,
    令牌: f64,
    速率: f64,
    上次: 时刻,
}

impl 瓶颈 {
    fn 新建(容量: f64, 速率: f64) -> Self {
        Self { 容量, 令牌: 容量, 速率, 上次: 时刻::此刻() }
    }

    fn 尝试获取(&mut self) -> bool {
        let 此刻 = 时刻::此刻();
        self.令牌 = (self.令牌 + 此刻.间隔(self.上次).秒数() * self.速率)
            .最小(self.容量);
        self.上次 = 此刻;

        if self.令牌 < 1.0 {
            return false;
        }
        self.令牌 -= 1.0;
        true
    }
}

fn main() {
    let mut 闸门 = 瓶颈::新建(10.0, 2.0); // 桶容量 10，每秒补 2 个令牌
    闸门.尝试获取(); // true
}
