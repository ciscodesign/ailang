#[cfg(test)]
mod tests {
    use crate::{Effect, EffectSet, CapToken};
    #[test]
    fn effect_set_union() {
        let a = EffectSet::of(&[Effect::Net, Effect::Db]);
        let b = EffectSet::of(&[Effect::Db, Effect::Llm]);
        let u = a.union(&b);
        assert!(u.contains(Effect::Net));
        assert!(u.contains(Effect::Db));
        assert!(u.contains(Effect::Llm));
        assert!(!u.contains(Effect::Fs));
    }
    #[test]
    fn effect_set_subset() {
        let small = EffectSet::of(&[Effect::Net]);
        let big   = EffectSet::of(&[Effect::Net, Effect::Db]);
        assert!(small.is_subset_of(&big));
        assert!(!big.is_subset_of(&small));
    }
    #[test]
    fn cap_token_sequences() {
        let t0 = CapToken::new(Effect::Net);
        assert_eq!(t0.seq(), 0);
        let t1 = t0.next();
        assert_eq!(t1.seq(), 1);
        let t2 = t1.next();
        assert_eq!(t2.seq(), 2);
    }
    #[test]
    fn cap_token_effect_preserved() {
        let t = CapToken::new(Effect::Db);
        assert_eq!(t.effect(), Effect::Db);
    }
}
