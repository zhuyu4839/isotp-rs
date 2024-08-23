use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Address {
    PrimaryEngineController,
    SecondaryEngineController,
    PrimaryTransmissionController,
    TransmissionShiftSelector,
    Brakes,
    Retarder,
    CruiseControl,
    FuelSystem,
    SteeringController,
    InstrumentCluster,
    ClimateControl1,
    Compass,
    BodyController,
    OffVehicleGateway,
    DidVid,
    RetarderExhaustEngine1,
    HeadwayController,
    Suspension,
    CabController,
    TirePressureController,
    LightingControlModule,
    ClimateControl2,
    ExhaustEmissionController,
    AuxiliaryHeater,
    ChassisController,
    CommunicationsUnit,
    Radio,
    SafetyRestraintSystem,
    AftertreatmentControlModule,
    MultiPurposeCamera,
    SwitchExpansionModule,
    AuxiliaryGaugeSwitchPack,
    Iteris,
    QualcommPeopleNetTranslatorBox,
    StandAloneRealTimeClock,
    CenterPanel1,
    CenterPanel2,
    CenterPanel3,
    CenterPanel4,
    CenterPanel5,
    WabcoOnGuardRadar,
    SecondaryInstrumentCluster,
    OffboardDiagnostics,
    Trailer3Bridge,
    Trailer2Bridge,
    Trailer1Bridge,
    SafetyDirectProcessor,
    ForwardRoadImageProcessor,
    LeftRearDoorPod,
    RightRearDoorPod,
    DoorController1,
    DoorController2,
    Tachograph,
    HybridSystem,
    AuxiliaryPowerUnit,
    ServiceTool,
    SourceAddressRequest0,
    SourceAddressRequest1,
    Unknown(u8),
}

impl From<u8> for Address {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::PrimaryEngineController,
            1 => Self::SecondaryEngineController,
            3 => Self::PrimaryTransmissionController,
            5 => Self::TransmissionShiftSelector,
            11 => Self::Brakes,
            15 => Self::Retarder,
            17 => Self::CruiseControl,
            18 => Self::FuelSystem,
            19 => Self::SteeringController,
            23 => Self::InstrumentCluster,
            25 => Self::ClimateControl1,
            28 => Self::Compass,
            33 => Self::BodyController,
            37 => Self::OffVehicleGateway,
            40 => Self::DidVid,
            41 => Self::RetarderExhaustEngine1,
            42 => Self::HeadwayController,
            47 => Self::Suspension,
            49 => Self::CabController,
            51 => Self::TirePressureController,
            55 => Self::LightingControlModule,
            58 => Self::ClimateControl2,
            61 => Self::ExhaustEmissionController,
            69 => Self::AuxiliaryHeater,
            71 => Self::ChassisController,
            74 => Self::CommunicationsUnit,
            76 => Self::Radio,
            83 => Self::SafetyRestraintSystem,
            85 => Self::AftertreatmentControlModule,
            127 => Self::MultiPurposeCamera,
            128 => Self::SwitchExpansionModule,
            132 => Self::AuxiliaryGaugeSwitchPack,
            139 => Self::Iteris,
            142 => Self::QualcommPeopleNetTranslatorBox,
            150 => Self::StandAloneRealTimeClock,
            151 => Self::CenterPanel1,
            152 => Self::CenterPanel2,
            153 => Self::CenterPanel3,
            154 => Self::CenterPanel4,
            155 => Self::CenterPanel5,
            160 => Self::WabcoOnGuardRadar,
            167 => Self::SecondaryInstrumentCluster,
            172 => Self::OffboardDiagnostics,
            184 => Self::Trailer3Bridge,
            192 => Self::Trailer2Bridge,
            200 => Self::Trailer1Bridge,
            209 => Self::SafetyDirectProcessor,
            232 => Self::ForwardRoadImageProcessor,
            233 => Self::LeftRearDoorPod,
            234 => Self::RightRearDoorPod,
            236 => Self::DoorController1,
            237 => Self::DoorController2,
            238 => Self::Tachograph,
            239 => Self::HybridSystem,
            247 => Self::AuxiliaryPowerUnit,
            249 => Self::ServiceTool,
            254 => Self::SourceAddressRequest0,
            255 => Self::SourceAddressRequest1,
            a => Self::Unknown(a),
        }
    }
}

impl From<Address> for u8 {
    fn from(value: Address) -> Self {
        match value {
            Address::PrimaryEngineController => 0,
            Address::SecondaryEngineController => 1,
            Address::PrimaryTransmissionController => 3,
            Address::TransmissionShiftSelector => 5,
            Address::Brakes => 11,
            Address::Retarder => 15,
            Address::CruiseControl => 17,
            Address::FuelSystem => 18,
            Address::SteeringController => 19,
            Address::InstrumentCluster => 23,
            Address::ClimateControl1 => 25,
            Address::Compass => 28,
            Address::BodyController => 33,
            Address::OffVehicleGateway => 37,
            Address::DidVid => 40,
            Address::RetarderExhaustEngine1 => 41,
            Address::HeadwayController => 42,
            Address::Suspension => 47,
            Address::CabController => 49,
            Address::TirePressureController => 51,
            Address::LightingControlModule => 55,
            Address::ClimateControl2 => 58,
            Address::ExhaustEmissionController => 61,
            Address::AuxiliaryHeater => 69,
            Address::ChassisController => 71,
            Address::CommunicationsUnit => 74,
            Address::Radio => 76,
            Address::SafetyRestraintSystem => 83,
            Address::AftertreatmentControlModule => 85,
            Address::MultiPurposeCamera => 127,
            Address::SwitchExpansionModule => 128,
            Address::AuxiliaryGaugeSwitchPack => 132,
            Address::Iteris => 139,
            Address::QualcommPeopleNetTranslatorBox => 142,
            Address::StandAloneRealTimeClock => 150,
            Address::CenterPanel1 => 151,
            Address::CenterPanel2 => 152,
            Address::CenterPanel3 => 153,
            Address::CenterPanel4 => 154,
            Address::CenterPanel5 => 155,
            Address::WabcoOnGuardRadar => 160,
            Address::SecondaryInstrumentCluster => 167,
            Address::OffboardDiagnostics => 172,
            Address::Trailer3Bridge => 184,
            Address::Trailer2Bridge => 192,
            Address::Trailer1Bridge => 200,
            Address::SafetyDirectProcessor => 209,
            Address::ForwardRoadImageProcessor => 232,
            Address::LeftRearDoorPod => 233,
            Address::RightRearDoorPod => 234,
            Address::DoorController1 => 236,
            Address::DoorController2 => 237,
            Address::Tachograph => 238,
            Address::HybridSystem => 239,
            Address::AuxiliaryPowerUnit => 247,
            Address::ServiceTool => 249,
            Address::SourceAddressRequest0 => 254,
            Address::SourceAddressRequest1 => 255,
            Address::Unknown(a) => a,
        }
    }
}

impl Display for Address {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::PrimaryEngineController => write!(f, "Primary Engine Controller | (CPC, ECM)"),
            Self::SecondaryEngineController => write!(f, "Secondary Engine Controller | (MCM, ECM #2)"),
            Self::PrimaryTransmissionController => write!(f, "Primary Transmission Controller | (TCM)"),
            Self::TransmissionShiftSelector => write!(f, "Transmission Shift Selector | (TSS)"),
            Self::Brakes => write!(f, "Brakes | System Controller (ABS)"),
            Self::Retarder => write!(f, "Retarder"),
            Self::CruiseControl => write!(f, "Cruise Control | (IPM, PCC)"),
            Self::FuelSystem => write!(f, "Fuel System | Controller (CNG)"),
            Self::SteeringController => write!(f, "Steering Controller | (SAS)"),
            Self::InstrumentCluster => write!(f, "Instrument Gauge Cluster (EGC) | (ICU, RX)"),
            Self::ClimateControl1 => write!(f, "Climate Control #1 | (FCU)"),
            Self::Compass => write!(f, "Compass"),
            Self::BodyController => write!(f, "Body Controller | (SSAM, SAM-CAB, BHM)"),
            Self::OffVehicleGateway => write!(f, "Off-Vehicle Gateway | (CGW)"),
            Self::DidVid => write!(f, "Vehicle Information Display | Driver Information Display"),
            Self::RetarderExhaustEngine1 => write!(f, "Retarder, Exhaust, Engine #1"),
            Self::HeadwayController => write!(f, "Headway Controller | (RDF) | (OnGuard)"),
            Self::Suspension => write!(f, "Suspension | System Controller (ECAS)"),
            Self::CabController => write!(f, "Cab Controller | Primary (MSF, SHM, ECC)"),
            Self::TirePressureController => write!(f, "Tire Pressure Controller | (TPMS)"),
            Self::LightingControlModule => write!(f, "Lighting Control Module | (LCM)"),
            Self::ClimateControl2 => write!(f, "Climate Control #2 | Rear HVAC | (ParkSmart)"),
            Self::ExhaustEmissionController => write!(f, "Exhaust Emission Controller | (ACM) | (DCU)"),
            Self::AuxiliaryHeater => write!(f, "Auxiliary Heater | (ACU)"),
            Self::ChassisController => write!(f, "Chassis Controller | (CHM, SAM-Chassis)"),
            Self::CommunicationsUnit => write!(f, "Communications Unit | Cellular (CTP, VT)"),
            Self::Radio => write!(f, "Radio"),
            Self::SafetyRestraintSystem => write!(f, "Safety Restraint System | Air Bag | (SRS)"),
            Self::AftertreatmentControlModule => write!(f, "Aftertreatment Control Module | (ACM)"),
            Self::MultiPurposeCamera => write!(f, "Multi-Purpose Camera | (MPC)"),
            Self::SwitchExpansionModule => write!(f, "Switch Expansion Module | (SEM #1)"),
            Self::AuxiliaryGaugeSwitchPack => write!(f, "Auxiliary Gauge Switch Pack | (AGSP3)"),
            Self::Iteris => write!(f, "Iteris"),
            Self::QualcommPeopleNetTranslatorBox => write!(f, "Qualcomm - PeopleNet Translator Box"),
            Self::StandAloneRealTimeClock => write!(f, "Stand-Alone Real Time Clock | (SART)"),
            Self::CenterPanel1 => write!(f, "Center Panel MUX Switch Pack #1"),
            Self::CenterPanel2 => write!(f, "Center Panel MUX Switch Pack #2"),
            Self::CenterPanel3 => write!(f, "Center Panel MUX Switch Pack #3"),
            Self::CenterPanel4 => write!(f, "Center Panel MUX Switch Pack #4"),
            Self::CenterPanel5 => write!(f, "Center Panel MUX Switch Pack #5"),
            Self::WabcoOnGuardRadar => write!(f, "Wabco OnGuard Radar | OnGuard Display | Collision Mitigation System"),
            Self::SecondaryInstrumentCluster => write!(f, "Secondary Instrument Cluster | (SIC)"),
            Self::OffboardDiagnostics => write!(f, "Offboard Diagnostics"),
            Self::Trailer3Bridge => write!(f, "Trailer #3 Bridge"),
            Self::Trailer2Bridge => write!(f, "Trailer #2 Bridge"),
            Self::Trailer1Bridge => write!(f, "Trailer #1 Bridge"),
            Self::SafetyDirectProcessor => write!(f, "Bendix Camera | Safety Direct Processor (SDP) Module"),
            Self::ForwardRoadImageProcessor => write!(f, "Forward Road Image Processor | PAM Module | Lane Departure Warning (LDW) Module | (VRDU)"),
            Self::LeftRearDoorPod => write!(f, "Left Rear Door Pod"),
            Self::RightRearDoorPod => write!(f, "Right Rear Door Pod"),
            Self::DoorController1 => write!(f, "Door Controller #1"),
            Self::DoorController2 => write!(f, "Door Controller #2"),
            Self::Tachograph => write!(f, "Tachograph | (TCO)"),
            Self::HybridSystem => write!(f, "Hybrid System"),
            Self::AuxiliaryPowerUnit => write!(f, "Auxiliary Power Unit | (APU)"),
            Self::ServiceTool => write!(f, "Service Tool"),
            Self::SourceAddressRequest0 => write!(f, "Source Address Request 0"),
            Self::SourceAddressRequest1 => write!(f, "Source Address Request 1"),
            Self::Unknown(num) => write!(f, "Unknown({num})"),
        }
    }
}

/// Represents the source address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceAddress {
    /// No source address.
    None,
    /// Source address with a specific value.
    Some(u8),
}

/// Represents the destination address.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DestinationAddress {
    /// No destination address.
    None,
    /// Destination address with a specific value.
    Some(u8),
}

impl SourceAddress {
    /// Lookup and translate the [`SourceAddress`] object.
    ///
    /// # Returns
    /// - `Some(Address)`: If generic J1939 address is known.
    /// - `None`: If the pdu specific bits do not contain a destination address.
    #[must_use]
    pub fn lookup(self) -> Option<Address> {
        match self {
            SourceAddress::Some(value) => Some(value.into()),
            SourceAddress::None => None,
        }
    }
}

impl DestinationAddress {
    /// Lookup and translate the [`DestinationAddress`] object.
    ///
    /// # Returns
    /// - `Some(Address)`: If generic J1939 address is known.
    /// - `None`: If the pdu specific bits do not contain a destination address.
    #[must_use]
    pub fn lookup(self) -> Option<Address> {
        match self {
            DestinationAddress::Some(value) => Some(value.into()),
            DestinationAddress::None => None,
        }
    }
}
