#[derive(Debug)]
pub enum Signal {
    Dot,
    Dash,
    ShortGap,
    LongGap,
}

pub fn signals_to_char(signals: &[Signal]) -> Option<char> {
    use Signal::*;

    match signals {
        &[Dot, Dash] => Some('a'),
        &[Dash, Dot, Dot, Dot] => Some('b'),
        _ => None,
    }
}

pub struct SignalProcessor {
    previous_signals: Vec<Signal>,
}

impl SignalProcessor {
    pub fn new() -> Self {
        SignalProcessor {
            previous_signals: vec![],
        }
    }

    pub fn next(&mut self, signal: Signal) -> Option<String> {
        use Signal::*;
        match signal {
            Dot | Dash => {
                self.previous_signals.push(signal);
                None
            }
            ShortGap => {
                let signals = self.previous_signals.drain(..).collect::<Vec<_>>();
                match signals_to_char(&signals) {
                    Some(c) => Some(format!("{}", c)),
                    None => Some("".to_string()),
                }
            }
            LongGap => {
                let signals = self.previous_signals.drain(..).collect::<Vec<_>>();
                match signals_to_char(&signals) {
                    Some(c) => Some(format!("{} ", c)),
                    None => Some(" ".to_string()),
                }
            }
        }
    }
}

fn main() {
    use Signal::*;

    let signal_list = vec![
        Dot, Dash, ShortGap, Dash, Dot, Dot, Dot, LongGap, Dot, Dash, ShortGap,
    ];

    let mut processor = SignalProcessor::new();
    signal_list.into_iter().for_each(|s| {
        println!("{:?}", processor.next(s));
    });
}
