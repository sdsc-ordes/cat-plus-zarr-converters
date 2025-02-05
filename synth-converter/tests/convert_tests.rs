use sophia_isomorphism::isomorphic_graphs;
use synth_converter::{convert::json_to_rdf, rdf::rdf_parser::parse_turtle_to_graph};

#[test]
fn test_convert_filtrate_action() {
    let json_data = r#"
        {
            "batchID": "23",
            "Actions": [
                {
                    "actionName": "filtrateAction",
                    "startTime": "2024-07-25T12:15:23",
                    "endingTime": "2024-07-25T12:16:50",
                    "methodName": "filtrate",
                    "equipmentName": "Chemspeed SWING XL",
                    "subEquipmentName": "Filtration unit",
                    "containerID": "1",
                    "containerBarcode": "1"
                }
            ]
        }
    "#;
    let result = json_to_rdf(json_data, "turtle");
    let expected_ttl = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX cat: <http://example.org/cat#>
        PREFIX schema: <https://schema.org/>
        PREFIX unit: <https://qudt.org/vocab/unit/>
        PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
        PREFIX qudt: <http://qudt.org/schema/qudt/>
        PREFIX alloqual: <http://purl.allotrope.org/ontologies/quality#>
        PREFIX purl: <http://purl.allotrope.org/ontologies/>
        PREFIX obo: <http://purl.obolibrary.org/obo/>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

        [] a cat:FiltrateAction;
        cat:containerBarcode "1";
        cat:containerID "1";
        cat:hasBatch [ a cat:Batch;
            schema:name "23"];
        cat:subEquipmentName "Filtration unit";
        allores:AFR_0001606 "filtrate";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:16:50"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:15:23"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_pressure_action() {
    let json_data = r#"
        {
            "batchID": "23",
            "Actions": [
                {
                    "actionName": "setPressureAction",
                    "pressureMeasurement": {
                        "value": 5,
                        "unit": "bar",
                        "errorMargin": {
                            "value": 1,
                            "unit": "bar"
                        }
                    },
                    "startTime": "2024-07-25T12:03:50",
                    "endingTime": "2024-07-25T12:04:05",
                    "methodName": "set_pressure",
                    "equipmentName": "Chemspeed SWING XL",
                    "subEquipmentName": "MTP_Pressure",
                    "containerID": "1",
                    "containerBarcode": "1"
                }
            ]
        }
    "#;
    let result = json_to_rdf(json_data, "turtle");
    let expected_ttl = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX cat: <http://example.org/cat#>
        PREFIX schema: <https://schema.org/>
        PREFIX unit: <https://qudt.org/vocab/unit/>
        PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
        PREFIX alloproc: <http://purl.allotrope.org/ontologies/process#>
        PREFIX qudt: <http://qudt.org/schema/qudt/>
        PREFIX alloqual: <http://purl.allotrope.org/ontologies/quality#>
        PREFIX purl: <http://purl.allotrope.org/ontologies/>
        PREFIX obo: <http://purl.obolibrary.org/obo/>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

        [] a cat:SetPressureAction;
        cat:containerBarcode "1";
        cat:containerID "1";
        cat:hasBatch [ a cat:Batch;
            schema:name "23"];
        cat:subEquipmentName "MTP_Pressure";
        alloproc:AFP_0002677 [
            cat:errorMargin [
                qudt:unit unit:Bar;
                qudt:value "1"^^xsd:double];
            qudt:unit unit:Bar;
            qudt:value "5"^^xsd:double];
        allores:AFR_0001606 "set_pressure";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:04:05"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:03:50"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_set_temperature_action() {
    let json_data = r#"
        {
            "batchID": "23",
            "Actions": [
                {
                    "actionName": "setTemperatureAction",
                    "speedShaker": {
                        "value": 152,
                        "unit": "rpm",
                        "errorMargin": {
                            "value": 5,
                            "unit": "rpm"
                        }
                    },
                    "temperatureTumbleStirrer": {
                        "value": 25,
                        "unit": "°C",
                        "errorMargin": {
                            "value": 2,
                            "unit": "°C"
                        }
                    },
                    "temperatureShaker": {
                        "value": 25,
                        "unit": "°C",
                        "errorMargin": {
                            "value": 1,
                            "unit": "°C"
                        }
                    },
                    "startTime": "2024-07-25T12:00:00",
                    "endingTime": "2024-07-25T12:00:02",
                    "methodName": "set_temperature",
                    "equipmentName": "Chemspeed SWING XL",
                    "subEquipmentName": "heater",
                    "containerID": "1",
                    "containerBarcode": "1"
                    }
                ]
            }
        "#;
    let result = json_to_rdf(json_data, "turtle");
    let expected_ttl = r#"
            PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
            PREFIX cat: <http://example.org/cat#>
            PREFIX schema: <https://schema.org/>
            PREFIX unit: <https://qudt.org/vocab/unit/>
            PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
            PREFIX alloproc: <http://purl.allotrope.org/ontologies/process#>
            PREFIX qudt: <http://qudt.org/schema/qudt/>
            PREFIX alloqual: <http://purl.allotrope.org/ontologies/quality#>
            PREFIX purl: <http://purl.allotrope.org/ontologies/>
            PREFIX obo: <http://purl.obolibrary.org/obo/>
            PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

            [] a cat:SetTemperatureAction;
            cat:containerBarcode "1";
            cat:containerID "1";
            cat:hasBatch [ a cat:Batch;
                schema:name "23"];
            cat:speedInRPM [
                cat:errorMargin [
                    qudt:unit unit:REV-PER-MIN;
                    qudt:value "5"^^xsd:double];
                qudt:unit unit:REV-PER-MIN;
                qudt:value "152"^^xsd:double];
            cat:subEquipmentName "heater";
            cat:temperatureShakerShape [
                cat:errorMargin [
                    qudt:unit unit:DEG-C;
                    qudt:value "1"^^xsd:double];
                qudt:unit unit:DEG-C;
                qudt:value "25"^^xsd:double];
            cat:temperatureTumbleStirrerShape [
                cat:errorMargin [
                    qudt:unit unit:DEG-C;
                    qudt:value "2"^^xsd:double];
                qudt:unit unit:DEG-C;
                qudt:value "25"^^xsd:double];
            allores:AFR_0001606 "set_temperature";
            allores:AFR_0001723 "Chemspeed SWING XL";
            allores:AFR_0002423 "2024-07-25T12:00:02"^^xsd:dateTime;
            allores:AFX_0000622 "2024-07-25T12:00:00"^^xsd:dateTime.
        "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_add_action() {
    let json_data = r#"
    {
        "batchID": "23",
        "Actions": [
            {
                "actionName": "AddAction",
                "speedShaker": {
                    "value": 152,
                    "unit": "rpm",
                    "errorMargin": {
                        "value": 1,
                        "unit": "rpm"
                    }
                },
                "equipmentName": "Chemspeed SWING XL",
                "subEquipmentName": "GDU-V",
                "hasContainerPositionAndQuantity": [
                    {
                        "position": "A1",
                        "containerID": "1",
                        "quantity": {
                            "value": 0.024,
                            "unit": "mg",
                            "errorMargin": {
                                "value": 0.001,
                                "unit": "mg"
                            }
                        }
                    },
                    {
                        "position": "B1",
                        "containerID": "1",
                        "quantity": {
                            "value": 0.034,
                            "unit": "mg",
                            "errorMargin": {
                                "value": 0.002,
                                "unit": "mg"
                            }
                        }
                    }
                ],
                "startTime": "2024-07-25T12:01:29",
                "endingTime": "2024-07-25T12:01:35",
                "methodName": "addition",
                "dispenseState": "Liquid",
                "dispenseType": "volume",
                "hasSample": {
                    "expectedDatum": {
                        "value": 2,
                        "unit": "mg"
                    },
                    "containerID": "18",
                    "containerBarcode": "18",
                    "vialID": "17",
                    "vialType": "storage vial",
                    "role": "reagent",
                    "hasSample": [
                        {
                            "sampleID": "124",
                            "role": "reagent",
                            "internalBarCode": "2",
                            "expectedDatum": {
                                "value": 5,
                                "unit": "mg"
                            },
                            "measuredQuantity": {
                                "value": 1,
                                "unit": "mg",
                                "errorMargin": {
                                    "value": 0.001,
                                    "unit": "mg"
                                }
                            },
                            "physicalState": "Liquid",
                            "hasChemical": {
                                "chemicalID": "134",
                                "chemicalName": "4-methoxybenzaldehyde",
                                "CASNumber": "123-11-5",
                                "molecularMass": {
                                    "value": 136.15,
                                    "unit": "g/mol"
                                },
                                "smiles": "COC1=CC=C(C=C1)C=O",
                                "Inchi": "1S/C8H8O2/c1-10-8-4-2-7(6-9)3-5-8/h2-6H,1H3",
                                "molecularFormula": "C8H8O2",
                                "density": {
                                    "value": 1.119,
                                    "unit": "g/mL"
                                }
                            }
                        }
                    ]
                }
            }
        ]
    }
    "#;
    let result = json_to_rdf(json_data, "turtle");
    let expected_ttl = r#"
    PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
    PREFIX cat: <http://example.org/cat#>
    PREFIX schema: <https://schema.org/>
    PREFIX unit: <https://qudt.org/vocab/unit/>
    PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
    PREFIX qudt: <http://qudt.org/schema/qudt/>
    PREFIX alloqual: <http://purl.allotrope.org/ontologies/quality#>
    PREFIX purl: <http://purl.allotrope.org/ontologies/>
    PREFIX obo: <http://purl.obolibrary.org/obo/>
    PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

    [] a cat:AddAction;
    cat:dispenseType "volume";
    cat:hasBatch [ a cat:Batch;
        schema:name "23"];
    cat:hasContainerPositionAndQuantity [ a cat:ContainerPositionAndQuantity;
        cat:containerID "1";
        allores:AFR_0002240 "B1";
        qudt:quantity [
            cat:errorMargin [
                qudt:unit unit:MilliGM;
                qudt:value "0.002"^^xsd:double];
            qudt:unit unit:MilliGM;
            qudt:value "0.034"^^xsd:double]],
        [ a cat:ContainerPositionAndQuantity;
        cat:containerID "1";
        allores:AFR_0002240 "A1";
        qudt:quantity [
            cat:errorMargin [
                qudt:unit unit:MilliGM;
                qudt:value "0.001"^^xsd:double];
            qudt:unit unit:MilliGM;
            qudt:value "0.024"^^xsd:double]];
    cat:hasSample [ a cat:Sample;
        cat:containerBarcode "18";
        cat:containerID "18";
        cat:expectedDatum [
            qudt:unit unit:MilliGM;
            qudt:value "2"^^xsd:double];
        cat:hasSample [ a cat:Sample;
            cat:expectedDatum [
                qudt:unit unit:MilliGM;
                qudt:value "5"^^xsd:double];
            cat:has_chemical [ a obo:CHEBI_25367;
                cat:casNumber "123-11-5";
                cat:chemicalName "4-methoxybenzaldehyde";
                purl:identifier "134";
                allores:AFR_0001952 "C8H8O2";
                allores:AFR_0002294 "136.15";
                allores:AFR_0002295 "COC1=CC=C(C=C1)C=O";
                allores:AFR_0002296 "1S/C8H8O2/c1-10-8-4-2-7(6-9)3-5-8/h2-6H,1H3";
                obo:PATO_0001019 [
                    qudt:unit unit:GM-PER-MilliL;
                    qudt:value "1.119"^^xsd:double]];
            cat:internalBarCode "2";
            cat:measuredQuantity [
                cat:errorMargin [
                    qudt:unit unit:MilliGM;
                    qudt:value "0.001"^^xsd:double];
                qudt:unit unit:MilliGM;
                qudt:value "1"^^xsd:double];
            cat:role "reagent";
            purl:identifier "124";
            alloqual:AFQ_0000111 "Liquid";
            allores:AFR_0002036 [
                cat:errorMargin [
                    qudt:unit unit:MOL-PER-L;
                    qudt:value "0.001"^^xsd:double];
                qudt:unit unit:MOL-PER-L;
                qudt:value "1"^^xsd:double]];
        cat:role "reagent";
        cat:vialShape "storage vial";
        allores:AFR_0002464 "17"];
    cat:speedInRPM [
        cat:errorMargin [
            qudt:unit unit:REV-PER-MIN;
            qudt:value "1"^^xsd:double];
        qudt:unit unit:REV-PER-MIN;
        qudt:value "152"^^xsd:double];
    cat:subEquipmentName "GDU-V";
    alloqual:AFQ_0000111 "Liquid";
    allores:AFR_0001606 "addition";
    allores:AFR_0001723 "Chemspeed SWING XL";
    allores:AFR_0002423 "2024-07-25T12:01:35"^^xsd:dateTime;
    allores:AFX_0000622 "2024-07-25T12:01:29"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_shake_action() {
    let json_data = r#"
        {
            "batchID": "23",
            "Actions": [
                {
                    "actionName": "shakeAction",
                    "speedTumbleStirrer": {
                        "value": 600,
                        "unit": "rpm",
                        "errorMargin": {
                            "value": 1,
                            "unit": "rpm"
                        }
                    },
                    "startTime": "2024-07-25T12:03:31",
                    "endingTime": "2024-07-25T12:15:20",
                    "methodName": "shake",
                    "temperatureTumbleStirrer": {
                        "value": 25,
                        "unit": "°C",
                        "errorMargin": {
                            "value": 1,
                            "unit": "°C"
                        }
                    },
                    "temperatureShaker": {
                        "value": 25,
                        "unit": "°C",
                        "errorMargin": {
                            "value": 2,
                            "unit": "°C"
                        }
                    },
                    "equipmentName": "Chemspeed SWING XL",
                    "subEquipmentName": "Tumble Stirrer",
                    "containerID": "1",
                    "containerBarcode": "1"
                }
            ]
        }
    "#;
    let result = json_to_rdf(json_data, "turtle");
    let expected_ttl = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX cat: <http://example.org/cat#>
        PREFIX schema: <https://schema.org/>
        PREFIX unit: <https://qudt.org/vocab/unit/>
        PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
        PREFIX alloproc: <http://purl.allotrope.org/ontologies/process#>
        PREFIX qudt: <http://qudt.org/schema/qudt/>
        PREFIX alloqual: <http://purl.allotrope.org/ontologies/quality#>
        PREFIX purl: <http://purl.allotrope.org/ontologies/>
        PREFIX obo: <http://purl.obolibrary.org/obo/>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

        [] a cat:ShakeAction;
        cat:containerBarcode "1";
        cat:containerID "1";
        cat:hasBatch [ a cat:Batch;
            schema:name "23"];
        cat:speedTumbleStirrerShape [
            cat:errorMargin [
                qudt:unit unit:REV-PER-MIN;
                qudt:value "1"^^xsd:double];
            qudt:unit unit:REV-PER-MIN;
            qudt:value "600"^^xsd:double];
        cat:subEquipmentName "Tumble Stirrer";
        cat:temperatureShakerShape [
            cat:errorMargin [
                qudt:unit unit:DEG-C;
                qudt:value "2"^^xsd:double];
            qudt:unit unit:DEG-C;
            qudt:value "25"^^xsd:double];
        cat:temperatureTumbleStirrerShape [
            cat:errorMargin [
                qudt:unit unit:DEG-C;
                qudt:value "1"^^xsd:double];
            qudt:unit unit:DEG-C;
            qudt:value "25"^^xsd:double];
        allores:AFR_0001606 "shake";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:15:20"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:03:31"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}

#[test]
fn test_convert_set_vacuum_action() {
    let json_data = r#"
        {
            "batchID": "23",
            "Actions": [
                {
                    "actionName": "setVacuumAction",
                    "vacuum": {
                        "value": 20,
                        "unit": "bar",
                        "errorMargin": {
                            "value": 0.5,
                            "unit": "bar"
                        }
                    },
                    "startTime": "2024-07-25T12:03:41",
                    "endingTime": "2024-07-25T12:03:50",
                    "methodName": "set_vacuum",
                    "equipmentName": "Chemspeed SWING XL",
                    "subEquipmentName": "vacuum",
                    "containerID": "1",
                    "containerBarcode": "1"
                }
            ]
        }
    "#;
    let result = json_to_rdf(json_data, "turtle");
    let expected_ttl = r#"
        PREFIX rdf: <http://www.w3.org/1999/02/22-rdf-syntax-ns#>
        PREFIX cat: <http://example.org/cat#>
        PREFIX schema: <https://schema.org/>
        PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
        PREFIX qudt: <http://qudt.org/schema/qudt/>
        PREFIX alloqual: <http://purl.allotrope.org/ontologies/quality#>
        PREFIX purl: <http://purl.allotrope.org/ontologies/>
        PREFIX obo: <http://purl.obolibrary.org/obo/>
        PREFIX xsd: <http://www.w3.org/2001/XMLSchema#>

        [] a cat:SetVacuumAction;
        cat:containerBarcode "1";
        cat:containerID "1";
        cat:hasBatch [ a cat:Batch;
            schema:name "23"];
        cat:subEquipmentName "vacuum";
        allores:AFR_0001606 "set_vacuum";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:03:50"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:03:41"^^xsd:dateTime.
    "#;
    let expected_graph = parse_turtle_to_graph(&expected_ttl).unwrap();
    let result_ttl = result.as_ref().unwrap().as_str();
    let result_graph = parse_turtle_to_graph(&result_ttl).unwrap();
    let graphs_match = isomorphic_graphs(&result_graph, &expected_graph);
    assert_eq!(graphs_match.unwrap(), true);
}
