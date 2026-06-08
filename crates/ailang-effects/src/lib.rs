use std::collections::BTreeSet;
#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, PartialOrd, Ord)]
pub enum Effect {
    Net,
    Db,
    Fs,
    Llm,
    Human,
    Clock,
    Rand,
    Ui,
}
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct EffectSet(BTreeSet<Effect>);
impl EffectSet {
    pub fn empty() -> Self {
        Self::default()
    }
    pub fn of(effects: &[Effect]) -> Self {
        let mut set = BTreeSet::new();
        for e in effects {
            set.insert(*e);
        }
        Self(set)
    }
    pub fn contains(&self, e: Effect) -> bool {
        self.0.contains(&e)
    }
    pub fn union(&self, other: &EffectSet) -> EffectSet {
        let mut res = self.0.clone();
        res.extend(other.0.iter().copied());
        Self(res)
    }
    pub fn is_subset_of(&self, other: &EffectSet) -> bool {
        self.0.is_subset(&other.0)
    }
    pub fn iter(&self) -> impl Iterator<Item = &Effect> {
        self.0.iter()
    }
}
pub struct CapToken {
    effect: Effect,
    seq: u64,
}
impl CapToken {
    pub fn new(effect: Effect) -> Self {
        Self { effect, seq: 0 }
    }
    pub fn next(self) -> Self {
        Self { effect: self.effect, seq: self.seq + 1 }
    }
    pub fn effect(&self) -> Effect {
        self.effect
    }
    pub fn seq(&self) -> u64 {
        self.seq
    }
}
#[cfg(test)]
mod tests;
