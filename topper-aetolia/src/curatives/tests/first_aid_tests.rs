mod firstaid_tests {
    use super::super::*;

    #[test]
    fn test_pill_priorities() {
        let priority_lines = vec![
            "Your affliction curing priorities:".into(),
            "1) pill:     [generosity]".into(),
            "".into(),
            "2) pill:     [paresis]".into(),
        ];
        let mut priorities = HashMap::new();
        priorities.insert(FType::Generosity, 1);
        priorities.insert(FType::Paresis, 2);
        let parsed_priorities = parse_priorities(&priority_lines);
        assert_eq!(parsed_priorities, priorities);
    }

    #[test]
    fn test_multi_priorities() {
        let priority_lines = vec![
            "Your affliction curing priorities:".into(),
            "1) pill:     [generosity]".into(),
            "   poultice: [weakvoid]".into(),
            "".into(),
            "2) pill:     [paresis, asthma]".into(),
        ];
        let mut priorities = HashMap::new();
        priorities.insert(FType::Generosity, 1);
        priorities.insert(FType::Weakvoid, 1);
        priorities.insert(FType::Paresis, 2);
        priorities.insert(FType::Asthma, 2);
        let parsed_priorities = parse_priorities(&priority_lines);
        assert_eq!(parsed_priorities, priorities);
    }

    #[test]
    fn test_many_priorities() {
        let priority_lines = vec![
            "Your affliction curing priorities:".into(),
            "1) pill:     [generosity]".into(),
            "   poultice: [weakvoid]".into(),
            "".into(),
            "2) pill:     [paresis, asthma]".into(),
            "6)  pipe:     [squelched]".into(),
            "    poultice: [shivering, frozen, gorged, effused_blood, blurry_vision,".into(),
            "               smashed_throat, right_arm_crippled, left_arm_crippled, cracked_ribs,"
                .into(),
            "               whiplash, backstrain, collapsed_lung, left_arm_dislocated,".into(),
            "               left_leg_dislocated, right_arm_dislocated, right_leg_dislocated,"
                .into(),
            "               sore_wrist, sore_ankle, muscle_spasms, heatspear]".into(),
            "    pill:     [sensitivity, rend, epilepsy, masochism, loneliness, haemophilia,"
                .into(),
            "               lethargy, vomiting, impairment, crippled, allergies,".into(),
            "               shaderot_body, shaderot_benign, shaderot_spirit, shaderot_heat,".into(),
            "               shaderot_wither]".into(),
        ];
        let mut priorities = HashMap::new();
        priorities.insert(FType::Generosity, 1);
        priorities.insert(FType::Weakvoid, 1);
        priorities.insert(FType::Paresis, 2);
        priorities.insert(FType::Asthma, 2);
        priorities.insert(FType::Squelched, 6);
        priorities.insert(FType::Shivering, 6);
        priorities.insert(FType::Frozen, 6);
        priorities.insert(FType::Gorged, 6);
        priorities.insert(FType::EffusedBlood, 6);
        priorities.insert(FType::BlurryVision, 6);
        priorities.insert(FType::SmashedThroat, 6);
        priorities.insert(FType::RightArmCrippled, 6);
        priorities.insert(FType::LeftArmCrippled, 6);
        priorities.insert(FType::CrackedRibs, 6);
        priorities.insert(FType::Whiplash, 6);
        priorities.insert(FType::Backstrain, 6);
        priorities.insert(FType::CollapsedLung, 6);
        priorities.insert(FType::LeftArmDislocated, 6);
        priorities.insert(FType::LeftLegDislocated, 6);
        priorities.insert(FType::RightArmDislocated, 6);
        priorities.insert(FType::RightLegDislocated, 6);
        priorities.insert(FType::SoreWrist, 6);
        priorities.insert(FType::SoreAnkle, 6);
        priorities.insert(FType::MuscleSpasms, 6);
        priorities.insert(FType::Heatspear, 6);
        priorities.insert(FType::Sensitivity, 6);
        priorities.insert(FType::Rend, 6);
        priorities.insert(FType::Epilepsy, 6);
        priorities.insert(FType::Masochism, 6);
        priorities.insert(FType::Loneliness, 6);
        priorities.insert(FType::Haemophilia, 6);
        priorities.insert(FType::Lethargy, 6);
        priorities.insert(FType::Vomiting, 6);
        priorities.insert(FType::Impairment, 6);
        priorities.insert(FType::Crippled, 6);
        priorities.insert(FType::Allergies, 6);
        priorities.insert(FType::ShaderotBody, 6);
        priorities.insert(FType::ShaderotBenign, 6);
        priorities.insert(FType::ShaderotSpirit, 6);
        priorities.insert(FType::ShaderotHeat, 6);
        priorities.insert(FType::ShaderotWither, 6);
        let parsed_priorities = parse_priorities(&priority_lines);
        assert_eq!(parsed_priorities, priorities);
    }

    #[test]
    fn test_strip_ansi() {
        let ansi_line = "\x1b[0;33m\x1b[48;5;232m1)\x1b[0;37m\x1b[48;5;232m  \x1b[0;32m\x1b[48;5;232mpipe:\x1b[0;37m\x1b[48;5;232m     \x1b[0;32m\x1b[48;5;232m[\x1b[0;37m\x1b[48;5;232maeon\x1b[0;32m\x1b[48;5;232m]\r\n";
        let stripped = strip_ansi(&ansi_line.to_string());
        assert_eq!(stripped, "1)  pipe:     [aeon]");
    }
}
