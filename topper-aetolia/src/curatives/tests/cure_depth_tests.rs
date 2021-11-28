mod cure_depth_tests {
    use super::super::*;

    #[test]
    fn test_pill() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::Clumsiness, true);
        agent.set_flag(FType::Asthma, true);
        let cure_depth = get_cure_depth(&agent, FType::Asthma);
        assert_eq!(cure_depth.affs, vec![FType::Clumsiness, FType::Asthma]);
        assert_eq!(cure_depth.time, 150);
        assert_eq!(cure_depth.cures, 2);
    }

    #[test]
    fn test_pill_off_bal() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::Clumsiness, true);
        agent.set_flag(FType::Asthma, true);
        agent.set_balance(BType::Pill, 1.0);
        let cure_depth = get_cure_depth(&agent, FType::Asthma);
        assert_eq!(cure_depth.affs, vec![FType::Clumsiness, FType::Asthma]);
        assert_eq!(cure_depth.time, 250);
        assert_eq!(cure_depth.cures, 2);
    }

    #[test]
    fn test_salve() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::MuscleSpasms, true);
        agent.set_flag(FType::Stiffness, true);
        let cure_depth = get_cure_depth(&agent, FType::Stiffness);
        assert_eq!(cure_depth.affs, vec![FType::MuscleSpasms, FType::Stiffness]);
        assert_eq!(cure_depth.time, 150);
        assert_eq!(cure_depth.cures, 2);
    }

    #[test]
    fn test_smoke_asthma() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::Disfigurement, true);
        agent.set_flag(FType::Asthma, true);
        let cure_depth = get_cure_depth(&agent, FType::Disfigurement);
        assert_eq!(cure_depth.affs, vec![FType::Asthma, FType::Disfigurement]);
        assert_eq!(cure_depth.time, 0);
        assert_eq!(cure_depth.cures, 2);
    }

    #[test]
    fn test_smoke_asthma_anorexia() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::Disfigurement, true);
        agent.set_flag(FType::Asthma, true);
        agent.set_flag(FType::Anorexia, true);
        let cure_depth = get_cure_depth(&agent, FType::Disfigurement);
        assert_eq!(
            cure_depth.affs,
            vec![FType::Anorexia, FType::Asthma, FType::Disfigurement]
        );
        assert_eq!(cure_depth.time, 0);
        assert_eq!(cure_depth.cures, 3);
    }

    #[test]
    fn test_locked() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::Slickness, true);
        agent.set_flag(FType::Asthma, true);
        agent.set_flag(FType::Anorexia, true);
        let cure_depth = get_cure_depth(&agent, FType::Slickness);
        assert_eq!(
            cure_depth.affs,
            vec![FType::Anorexia, FType::Asthma, FType::Slickness]
        );
        assert_eq!(cure_depth.time, 0);
        assert_eq!(cure_depth.cures, 3);
    }

    #[test]
    fn test_aeon() {
        let mut agent = AgentState::default();
        agent.set_flag(FType::Clumsiness, true);
        agent.set_flag(FType::Asthma, true);
        agent.set_flag(FType::Aeon, true);
        let cure_depth = get_cure_depth(&agent, FType::Aeon);
        assert_eq!(
            cure_depth.affs,
            vec![FType::Clumsiness, FType::Asthma, FType::Aeon]
        );
        assert_eq!(cure_depth.time, 150);
        assert_eq!(cure_depth.cures, 3);
    }
}