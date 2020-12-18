#![feature(external_doc)]
use ta_common::traits::Indicator;

#[doc(include = "../README.md")]
pub struct KVO {
    prev_hlc: f64,
    prev_dmt: Option<f64>,
    prev_trend: f64,
    prev_cmt: f64,
    prev_short_vft: f64,
    prev_long_vft: f64,
    long_period: u32,
    short_period: u32,
    long_factor: f64,
    short_factor: f64,
    long_factor_compliment: f64,
    short_factor_compliment: f64,
}


impl KVO {
    pub fn new(short: u32, long: u32) -> KVO {
        let long_factor = 2.0 / (long + 1) as f64;
        let short_factor = 2.0 / (short + 1) as f64;
        let long_factor_compliment = 1.0 - long_factor;
        let short_factor_compliment = 1.0 - short_factor;
        Self {
            prev_hlc: 0.0,
            prev_dmt: None,
            prev_trend: 0.0,
            prev_cmt: 0.0,
            prev_short_vft: 0.0,
            prev_long_vft: 0.0,
            long_period: long,
            short_period: short,
            long_factor,
            long_factor_compliment,
            short_factor,
            short_factor_compliment,
        }
    }
    fn get_trend(&self, hlc: f64) -> f64 {
        return if hlc > self.prev_hlc {
            1.0
        } else {
            -1.0
        }
    }

    fn get_cmt(&self, trend: f64, prev_dmt: f64, dmt: f64) -> f64 {
        return if trend != self.prev_trend {
            prev_dmt + dmt
        } else {
            self.prev_cmt + dmt
        };
    }
}

impl Indicator<[f64; 4], Option<f64>> for KVO {
    fn next(&mut self, input: [f64; 4]) -> Option<f64> {
        let [high, low, close, volume] = input;
        let hlc = high + low + close;
        let dmt = high - low;
        let res = match self.prev_dmt {
            None => None,
            Some(prev_dmt) => {
                let trend = self.get_trend(hlc);
                let cmt = self.get_cmt(trend, prev_dmt, dmt);
                let vft = 100.0 * volume * trend * (2.0*(dmt / cmt  - 1.0)).abs();
                let short_vft = self.short_factor_compliment * self.prev_short_vft + self.short_factor * vft;
                let long_vft = self.long_factor_compliment * self.prev_long_vft + self.long_factor * vft;
                let kvo = short_vft - long_vft;
              //  println!("hlc {} cmt {} trend{} vft{} short_vft{} long_vft{} kvo {}", hlc,cmt,trend,vft, short_vft, long_vft, kvo);
                self.prev_long_vft = long_vft;
                self.prev_short_vft = short_vft;
                self.prev_cmt = cmt;
                self.prev_trend = trend;
                Some(kvo)
            }
        };


        self.prev_dmt = Some(dmt);
        self.prev_hlc=hlc;
        res
    }


    fn reset(&mut self) {
        self.prev_hlc = 0.0;
        self.prev_dmt = None;
        self.prev_trend = -1.0;
        self.prev_cmt = 0.0;
        self.prev_short_vft = 0.0;
        self.prev_long_vft = 0.0;
    }
}


#[cfg(test)]
mod tests {
    use crate::KVO;
    use ta_common::traits::Indicator;

    #[test]
    fn it_works() {
        let mut kvo = KVO::new(2, 5);
        assert_eq!(kvo.next([82.15, 81.29, 81.59, 5_653_100.00]), None);
        assert_eq!(kvo.next([81.89, 80.64, 81.06, 6_447_400.00]), Some(-175190015.79778826));
        assert_eq!(kvo.next([83.03, 81.31, 82.87, 7_690_900.00]), Some(215794051.62738508));
        assert_eq!(kvo.next([83.30, 82.65, 83.00, 3_831_400.00]), Some(248493877.05267042));
        assert_eq!(kvo.next([83.85, 83.07, 83.61, 4_455_100.00]), Some(235332365.7752409));
        assert_eq!(kvo.next([83.90, 83.11, 83.15, 3_798_000.00]), Some(-190037611.21375185));
        assert_eq!(kvo.next([83.33, 82.49, 82.84, 3_936_200.00]), Some(-287489826.79249376));
        assert_eq!(kvo.next([84.30, 82.30, 83.99, 4_732_000.00]), Some(18997560.599353276));
        assert_eq!(kvo.next([84.84, 84.15, 84.55, 4_841_300.00]), Some(249242717.81561512));
        assert_eq!(kvo.next([85.00, 84.11, 84.36, 3_915_300.00]), Some(-128634280.44669077));
        assert_eq!(kvo.next([85.90, 84.03, 85.53, 6_830_800.00]), Some(76813896.24615139));
        assert_eq!(kvo.next([86.58, 85.39, 86.54, 6_694_100.00]), Some(270380021.79762703));
        assert_eq!(kvo.next([86.98, 85.76, 86.89, 5_293_600.00]), Some(211112721.2870996));
        assert_eq!(kvo.next([88.00, 87.17, 87.77, 7_985_800.00]), Some(340139231.8525958));
        assert_eq!(kvo.next([87.87, 87.01, 87.29, 4_807_900.00]), Some(-322932972.7463783));
    }
}
