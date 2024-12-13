use lazy_static::lazy_static;
use sophia::api::ns::Namespace;
use sophia_api::namespace;
namespace! {
    "http://example.org/cat#",
    AddAction,
    Batch,
    ContainerPositionAndQuantity,
    FiltrateAction,
    Sample,
    SetPressureAction,
    SetTemperatureAction,
    SetVacuumAction,
    ShakeAction,
    casNumber,
    chemicalName,
    containerBarcode,
    containerID,
    dispenseType,
    errorMargin,
    expectedDatum,
    hasBatch,
    hasContainerPositionAndQuantity,
    hasSample,
    has_chemical,
    internalBarCode,
    measuredQuantity,
    role,
    setTemperatureAction,
    speedInRPM,
    subEquipmentName,
    swissCatNumber,
    temperatureShakerShape,
    temperatureTumbleStirrerShape,
    vialShape
}
lazy_static! {
    pub static ref ns: Namespace<&'static str> = Namespace::new(PREFIX.as_str()).unwrap();
}
