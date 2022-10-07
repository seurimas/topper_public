use std::collections::HashMap;

pub fn add_mappings(mut mapping: &mut HashMap<(String, String), (String, String)>) {
    // Brutality/Zeal
    mapping.insert(
        ("Brutality".to_string(), "Wounds".to_string()),
        ("Zeal".to_string(), "Wounds".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Attitude".to_string()),
        ("Zeal".to_string(), "Mindset".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Oppose".to_string()),
        ("Zeal".to_string(), "Fending".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Sprint".to_string()),
        ("Zeal".to_string(), "Sprint".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Premeditation".to_string()),
        ("Zeal".to_string(), "Endlink".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Onslaught".to_string()),
        ("Zeal".to_string(), "Flow".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Velocity".to_string()),
        ("Zeal".to_string(), "Haste".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Predation".to_string()),
        ("Zeal".to_string(), "Wrath".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Exhilarate".to_string()),
        ("Zeal".to_string(), "Disunion".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Contempt".to_string()),
        ("Zeal".to_string(), "Swagger".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Physique".to_string()),
        ("Zeal".to_string(), "Caeveir".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Assail".to_string()),
        ("Zeal".to_string(), "Hackles".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Seeth".to_string()),
        ("Zeal".to_string(), "Respiration".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Contusion".to_string()),
        ("Zeal".to_string(), "Welt".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Bait".to_string()),
        ("Zeal".to_string(), "Rebuke".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Reflexes".to_string()),
        ("Zeal".to_string(), "Litheness".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Rampage".to_string()),
        ("Zeal".to_string(), "Blitz".to_string()),
    );
    // Fist
    mapping.insert(
        ("Brutality".to_string(), "Bully".to_string()),
        ("Zeal".to_string(), "Pummel".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Clobber".to_string()),
        ("Zeal".to_string(), "Palmforce".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Plexus".to_string()),
        ("Zeal".to_string(), "Clawtwist".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Bustup".to_string()),
        ("Zeal".to_string(), "Dislocate".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "PressurePoint".to_string()),
        ("Zeal".to_string(), "Twinpress".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Haymaker".to_string()),
        ("Zeal".to_string(), "Direblow".to_string()),
    );
    // Foot
    mapping.insert(
        ("Brutality".to_string(), "Kneecap".to_string()),
        ("Zeal".to_string(), "Wanekick".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Concussion".to_string()),
        ("Zeal".to_string(), "Sunkick".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Rebound".to_string()),
        ("Zeal".to_string(), "Risekick".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Overpower".to_string()),
        ("Zeal".to_string(), "Heelrush".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Windpipe".to_string()),
        ("Zeal".to_string(), "Edgekick".to_string()),
    );
    // Hook
    mapping.insert(
        ("Brutality".to_string(), "Hobble".to_string()),
        ("Zeal".to_string(), "Anklepin".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Slug".to_string()),
        ("Zeal".to_string(), "Jawcrack".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Flog".to_string()),
        ("Zeal".to_string(), "Descent".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Main".to_string()),
        ("Zeal".to_string(), "Wristlash".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Butcher".to_string()),
        ("Zeal".to_string(), "Rive".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Whiplash".to_string()),
        ("Zeal".to_string(), "Uprise".to_string()),
    );
    mapping.insert(
        ("Brutality".to_string(), "Tenderise".to_string()),
        ("Zeal".to_string(), "Whipburst".to_string()),
    );
    // Ravaging/Purification
    mapping.insert(
        ("Ravaging".to_string(), "Vinculum".to_string()),
        ("Purification".to_string(), "Focalmark".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Wither".to_string()),
        ("Purification".to_string(), "Ignition".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Torment".to_string()),
        ("Purification".to_string(), "Scorch".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Suffering".to_string()),
        ("Purification".to_string(), "Intensity".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Prolong".to_string()),
        ("Purification".to_string(), "Cauterize".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Smoulder".to_string()),
        ("Purification".to_string(), "Hearth".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Impressment".to_string()),
        ("Purification".to_string(), "Cinderkin".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Hood".to_string()),
        ("Purification".to_string(), "Suncloak".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Begrudge".to_string()),
        ("Purification".to_string(), "Cascade".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Ruthlessness".to_string()),
        ("Purification".to_string(), "Tempering".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Ravage".to_string()),
        ("Purification".to_string(), "Firefist".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Denial".to_string()),
        ("Purification".to_string(), "Rejection".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Intensify".to_string()),
        ("Purification".to_string(), "Quicken".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Infuse".to_string()),
        ("Purification".to_string(), "Etching".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Bedevil".to_string()),
        ("Purification".to_string(), "Pendulum".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Impenetrable".to_string()),
        ("Purification".to_string(), "Deflection".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Lancing".to_string()),
        ("Purification".to_string(), "Heatspear".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Criticality".to_string()),
        ("Purification".to_string(), "Discharge".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Branding".to_string()),
        ("Purification".to_string(), "Infernal".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Delerium".to_string()),
        ("Purification".to_string(), "Zenith".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Shear".to_string()),
        ("Purification".to_string(), "Exudation".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Extinguish".to_string()),
        ("Purification".to_string(), "Immolation".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Unfinished".to_string()),
        ("Purification".to_string(), "Resurgence".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Bedlam".to_string()),
        ("Purification".to_string(), "Dwindle".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Hellfire".to_string()),
        ("Purification".to_string(), "Pyromania".to_string()),
    );
    mapping.insert(
        ("Ravaging".to_string(), "Contest".to_string()),
        ("Purification".to_string(), "Gauntlet".to_string()),
    );
    // Egotism/Psionics
    mapping.insert(
        ("Egotism".to_string(), "Check".to_string()),
        ("Psionics".to_string(), "Might".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Deathsight".to_string()),
        ("Psionics".to_string(), "Deathsight".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Aura".to_string()),
        ("Psionics".to_string(), "Aura".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Awareness".to_string()),
        ("Psionics".to_string(), "Waves".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Moderate".to_string()),
        ("Psionics".to_string(), "Regulate".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Pinpoint".to_string()),
        ("Psionics".to_string(), "Locate".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Haul".to_string()),
        ("Psionics".to_string(), "Fetch".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Peak".to_string()),
        ("Psionics".to_string(), "Flash".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Reconcile".to_string()),
        ("Psionics".to_string(), "Harmonize".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Belittle".to_string()),
        ("Psionics".to_string(), "Medium".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Redress".to_string()),
        ("Psionics".to_string(), "Neutralise".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Trip".to_string()),
        ("Psionics".to_string(), "Lance".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Exhibit".to_string()),
        ("Psionics".to_string(), "Hold".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Inadequacy".to_string()),
        ("Psionics".to_string(), "Dull".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Boost".to_string()),
        ("Psionics".to_string(), "Recover".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Boast".to_string()),
        ("Psionics".to_string(), "Deprival".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Expose".to_string()),
        ("Psionics".to_string(), "Psychometry".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Reign".to_string()),
        ("Psionics".to_string(), "Flight".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Monopolise".to_string()),
        ("Psionics".to_string(), "Absorption".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Sizeup".to_string()),
        ("Psionics".to_string(), "Fathom".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Dethrone".to_string()),
        ("Psionics".to_string(), "Vacuum".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Narcissism".to_string()),
        ("Psionics".to_string(), "Warning".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Trauma".to_string()),
        ("Psionics".to_string(), "Shock".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Menace".to_string()),
        ("Psionics".to_string(), "Dread".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Pry".to_string()),
        ("Psionics".to_string(), "Sliver".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Inflate".to_string()),
        ("Psionics".to_string(), "Mindspark".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Kneel".to_string()),
        ("Psionics".to_string(), "Tether".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Skip".to_string()),
        ("Psionics".to_string(), "Riftwalk".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Untouchable".to_string()),
        ("Psionics".to_string(), "Bending".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Press".to_string()),
        ("Psionics".to_string(), "Traject".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Guts".to_string()),
        ("Psionics".to_string(), "Torrent".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Humiliate".to_string()),
        ("Psionics".to_string(), "Corporality".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Narrow".to_string()),
        ("Psionics".to_string(), "Step".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Outlaw".to_string()),
        ("Psionics".to_string(), "Disable".to_string()),
    );
    mapping.insert(
        ("Egotism".to_string(), "Double".to_string()),
        ("Psionics".to_string(), "Projection".to_string()),
    );
}
