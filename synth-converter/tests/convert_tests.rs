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
                "subEquipmentName": "Filtration station",
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

        [] a allores:AFRE_0000001;
        cat:containerBarcode "1";
        cat:containerID "1";
        cat:hasBatch [ a cat:Batch;
            schema:name "23"];
        cat:localEquipmentName "Filtration station";
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
        PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
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
        PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
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
                qudt:unit "rpm";
                qudt:value "5"^^xsd:double];
            qudt:unit "rpm";
            qudt:value "152"^^xsd:double];
        cat:subEquipmentName "heater";
        cat:temperatureShakerShape [
            cat:errorMargin [
                qudt:unit "°C";
                qudt:value "1"^^xsd:double];
            qudt:unit "°C";
            qudt:value "25"^^xsd:double];
        cat:temperatureTumbleStirrerShape [
            cat:errorMargin [
                qudt:unit "°C";
                qudt:value "2"^^xsd:double];
            qudt:unit "°C";
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
                    "unit": "rpm"
                },
                "equipmentName": "Chemspeed SWING XL",
                "subEquipmentName": "GDU-V",
                "hasContainerPositionAndQuantity": [
                    {
                        "position": "1A1",
                        "quantity": {
                            "value": 1,
                            "unit": "mg"
                        }
                    },
                    {
                        "position": "1B1",
                        "quantity": {
                            "value": 4,
                            "unit": "mg"
                        }
                    }
                ],
                "startTime": "2024-07-25T12:00:13",
                "endingTime": "2024-07-25T12:00:17",
                "methodName": "addition",
                "dispenseState": "Liquid",
                "dispenseType": "volume",
                "hasSample": {
                    "expectedDatum": {
                        "value": 2,
                        "unit": "mg"
                    },
                    "containerID": "17",
                    "containerBarcode": "17",
                    "vialID": "15",
                    "vialType": "storage vial",
                    "role": "solvent",
                    "hasSample": [
                        {
                            "sampleID": "123",
                            "role": "solvent",
                            "internalBarCode": "1",
                            "measuredQuantity": {
                                "value": "",
                                "unit": "mg"
                            },
                            "physicalState": "Liquid",
                            "concentration": {
                                "value": "",
                                "unit": "mol/L"
                            },
                            "purity": "",
                            "hasChemical": {
                                "chemicalID": "134",
                                "chemicalName": "Toluène",
                                "CASNumber": "108-88-3",
                                "molecularMass": {
                                    "value": 92.14,
                                    "unit": "g/mol"
                                },
                                "smiles": "CC1=CC=CC=C1"
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
        cat:hasSample [ a cat:Sample;
            cat:containerBarcode "17";
            cat:containerID "17";
            cat:expectedDatum [
                qudt:unit "mg";
                qudt:value "2"^^xsd:double];
            cat:hasSample [ a cat:Sample;
                cat:has_chemical [ a obo:CHEBI_25367;
                    cat:casNumber "108-88-3";
                    cat:chemicalName "Toluène";
                    purl:identifier "134";
                    allores:AFR_0002294 "92.14";
                    allores:AFR_0002295 "CC1=CC=CC=C1"];
                cat:internalBarCode "1";
                cat:role "solvent";
                purl:identifier "123";
                alloqual:AFQ_0000111 "Liquid"];
            cat:role "solvent";
            cat:vialShape "storage vial";
            allores:AFR_0002464 "15"];
        cat:localEquipmentName "GDU-V";
        cat:speedInRPM [
            qudt:unit "rpm";
            qudt:value "152"^^xsd:double];
        alloqual:AFQ_0000111 "Liquid";
        allores:AFR_0001606 "addition";
        allores:AFR_0001723 "Chemspeed SWING XL";
        allores:AFR_0002423 "2024-07-25T12:00:17"^^xsd:dateTime;
        allores:AFX_0000622 "2024-07-25T12:00:13"^^xsd:dateTime.
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
        PREFIX allores: <http://purl.allotrope.org/ontologies/result#>
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
        cat:subEquipmentName "Tumble Stirrer";
        cat:temperatureShakerShape [
            cat:errorMargin [
                qudt:unit "°C";
                qudt:value "2"^^xsd:double];
            qudt:unit "°C";
            qudt:value "25"^^xsd:double];
        cat:temperatureTumbleStirrerShape [
            cat:errorMargin [
                qudt:unit "°C";
                qudt:value "1"^^xsd:double];
            qudt:unit "°C";
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
