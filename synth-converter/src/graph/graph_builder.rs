use crate::{
    batch::{
        Action, ActionName, Batch, Chemical, ContainerInfo, ContainerPositionQuantityItem,
        Observation, Sample, SampleItem, ErrorMargin
    },
    graph::{
        namespaces::{alloqual, allores, cat, obo, purl, qudt, schema, unit},
        utils::generate_bnode_term,
    },
    rdf::rdf_serializers::{serialize_graph_to_jsonld, serialize_graph_to_turtle},
};
use anyhow::{Context, Result};
use sophia::{
    api::{
        graph::MutableGraph,
        ns::{rdf, xsd},
    },
    inmem::graph::LightGraph,
};
use sophia_api::{ns::NsTerm, term::SimpleTerm};

/// An RDF Graph
pub struct GraphBuilder {
    graph: LightGraph,
}

/// Builds an RDF graph of Synthesis data for the cat+ ontology.
///
/// The rust structure `actions` in /parser/actions is mapped to the cat+ ontology
///
/// # public methods:
/// * insert_a_batch:  starts the process of building the graph from the input structure
/// * serialize_to_turtle: serializes the graph to a turtle output
impl GraphBuilder {
    pub fn new() -> Result<Self> {
        Ok(Self {
            graph: LightGraph::new(),
        })
    }

    fn insert_a_date_time(
        &mut self,
        subject: &SimpleTerm,
        predicate: &NsTerm<'_>,
        date_time: &str,
    ) -> Result<()> {
        let object = date_time * xsd::dateTime;
        self.graph.insert(subject, predicate, &object)?;

        Ok(())
    }

    fn insert_container_properties(
        &mut self,
        subject: &SimpleTerm,
        container_info: &ContainerInfo,
    ) -> Result<()> {
        self.graph.insert(
            subject,
            cat::containerID,
            container_info.container_id.as_str(),
        )?;
        self.graph.insert(
            subject,
            cat::containerBarcode,
            container_info.container_barcode.as_str(),
        )?;

        Ok(())
    }

    fn insert_an_observation(
        &mut self,
        subject: &SimpleTerm,
        property_term: &NsTerm<'_>,
        observation: &Observation,
    ) -> Result<()> {
        let observation_term = generate_bnode_term();

        self.graph
            .insert(subject, property_term, &observation_term)?;
        self.graph
            .insert(&observation_term, qudt::unit, observation.unit.as_str())?;
        self.graph
            .insert(&observation_term, qudt::value, observation.value)?;

        if let Some(error_margin) = &observation.error_margin {
            self.insert_an_error_margin(&observation_term, &cat::errorMargin, error_margin)?;
        }

        Ok(())
    }

    fn insert_a_density(
        &mut self,
        subject: &SimpleTerm,
        property_term: &NsTerm<'_>,
        observation: &Observation,
    ) -> Result<()> {
        let density_term = generate_bnode_term();

        self.graph
            .insert(subject, property_term, &density_term)?;
        self.graph
            .insert(&density_term, qudt::unit, unit::MilliGM)?;
        self.graph
            .insert(&density_term, qudt::value, observation.value)?;

        Ok(())
    }

    fn insert_an_error_margin(
        &mut self,
        subject: &SimpleTerm,
        property_term: &NsTerm<'_>,
        error_margin: &ErrorMargin,
    ) -> Result<()> {
        let error_margin_term = generate_bnode_term();

        self.graph
            .insert(subject, property_term, &error_margin_term)?;
        self.graph
            .insert(&error_margin_term, qudt::unit, error_margin.unit.as_str())?;
        self.graph
            .insert(&error_margin_term, qudt::value, error_margin.value)?;

        Ok(())
    }

    fn insert_a_container_position_and_quantity(
        &mut self,
        subject: &SimpleTerm,
        container_position_quantity_item: &ContainerPositionQuantityItem,
    ) -> Result<()> {
        let container_position_quantity_item_term = generate_bnode_term();

        self.graph.insert(
            subject,
            cat::hasContainerPositionAndQuantity,
            &container_position_quantity_item_term,
        )?;
        self.graph.insert(
            &container_position_quantity_item_term,
            rdf::type_,
            cat::ContainerPositionAndQuantity,
        )?;
        self.graph.insert(
            &container_position_quantity_item_term,
            allores::AFR_0002240,
            container_position_quantity_item.position.as_str(),
        )?;
        self.graph.insert(
            &container_position_quantity_item_term,
            cat::containerID,
            container_position_quantity_item.container_id.as_str(),
        )?;

        self.insert_an_observation(
            &container_position_quantity_item_term,
            &qudt::quantity,
            &container_position_quantity_item.quantity,
        )?;

        Ok(())
    }

    fn insert_a_chemical(&mut self, subject: &SimpleTerm, chemical: &Chemical) -> Result<()> {
        let chemical_term: SimpleTerm = generate_bnode_term();

        self.graph
            .insert(subject, cat::has_chemical, &chemical_term)?;
        self.graph
            .insert(&chemical_term, rdf::type_, obo::CHEBI_25367)?;
        self.graph.insert(
            &chemical_term,
            purl::identifier,
            chemical.chemical_id.as_str(),
        )?;
        self.graph.insert(
            &chemical_term,
            cat::chemicalName,
            chemical.chemical_name.as_str(),
        )?;
        if let Some(cas_number) = &chemical.cas_number {
            self.graph
                .insert(&chemical_term, cat::casNumber, cas_number.as_str())?;
        }
        self.graph.insert(
            &chemical_term,
            allores::AFR_0002295,
            chemical.smiles.as_str(),
        )?;
        let molecular_mass = chemical.molecular_mass.value.to_string();
        self.graph
            .insert(&chemical_term, allores::AFR_0002294, &*molecular_mass)?;

        self.graph
            .insert(&chemical_term, allores::AFR_0002296, chemical.inchi.as_str())?;
        self.graph
            .insert(&chemical_term, allores::AFR_0001952, chemical.molecular_formula.as_str())?;
        if let Some(swiss_cat_number) = &chemical.swiss_cat_number {
            self.graph
                .insert(&chemical_term, cat::swissCatNumber, swiss_cat_number.as_str())?;
        }
        if let Some(keywords) = &chemical.keywords {
            self.graph
                .insert(&chemical_term, schema::keywords, keywords.as_str())?;
        }

        if let Some(density) = &chemical.density {
            self.insert_a_density(
                &chemical_term,
                &obo::PATO_0001019,
                density,
            )?;
        }

        Ok(())
    }

    fn insert_a_sample(&mut self, subject: &SimpleTerm, sample_item: &SampleItem) -> Result<()> {
        let sample_item_term = generate_bnode_term();

        self.graph
            .insert(&sample_item_term, rdf::type_, cat::Sample)?;
        self.graph
            .insert(subject, cat::hasSample, &sample_item_term)?;
        self.graph
            .insert(&sample_item_term, cat::role, sample_item.role.as_str())?;

        if let Some(expected_datum) = &sample_item.expected_datum {
            self.insert_an_observation(&sample_item_term, &cat::expectedDatum, expected_datum)?;
        }

        if let Some(measured_quantity) = &sample_item.measured_quantity {
            self.insert_an_observation(&sample_item_term, &cat::measuredQuantity, measured_quantity)?;
        }

        if let Some(concentration) = &sample_item.measured_quantity {
            self.insert_an_observation(&sample_item_term, &allores::AFR_0002036, concentration)?;
        }


        self.graph
            .insert(&sample_item_term, cat::role, sample_item.role.as_str())?;
        self.graph.insert(
            &sample_item_term,
            purl::identifier,
            sample_item.sample_id.as_str(),
        )?;
        self.graph.insert(
            &sample_item_term,
            alloqual::AFQ_0000111,
            sample_item.physical_state.as_str(),
        )?;
        self.graph.insert(
            &sample_item_term,
            cat::internalBarCode,
            sample_item.internal_bar_code.as_str(),
        )?;
        self.insert_a_chemical(&sample_item_term, &sample_item.has_chemical)?;

        Ok(())
    }

    fn insert_samples(&mut self, subject: &SimpleTerm, sample: &Sample) -> Result<()> {
        let sample_term = generate_bnode_term();

        self.graph.insert(subject, cat::hasSample, &sample_term)?;
        self.graph.insert(&sample_term, rdf::type_, cat::Sample)?;

        self.insert_container_properties(&sample_term, &sample.container)?;

        self.insert_an_observation(&sample_term, &cat::expectedDatum, &sample.expected_datum)?;

        self.graph
            .insert(&sample_term, cat::vialShape, sample.vial_type.as_str())?;
        self.graph
            .insert(&sample_term, allores::AFR_0002464, sample.vial_id.as_str())?;

        self.graph
            .insert(&sample_term, cat::role, sample.role.as_str())?;

        for sample_item in &sample.has_sample {
            self.insert_a_sample(&sample_term, &sample_item)
                .context("Failed to insert sample")?
        }

        Ok(())
    }

    fn insert_action_type(&mut self, subject: &SimpleTerm, action: &Action) -> Result<()> {
        match action.action_name {
            ActionName::AddAction => {
                self.graph.insert(subject, rdf::type_, cat::AddAction)?;
            }

            ActionName::setTemperatureAction => {
                self.graph
                    .insert(subject, rdf::type_, cat::SetTemperatureAction)?;
            }

            ActionName::setPressureAction => {
                self.graph
                    .insert(subject, rdf::type_, cat::SetPressureAction)?;
            }

            ActionName::shakeAction => {
                self.graph
                    .insert(subject, rdf::type_, cat::ShakeAction)?;
            }

            ActionName::setVacuumAction => {
                self.graph
                    .insert(subject, rdf::type_, cat::SetVacuumAction)?;
            }

            ActionName::filtrateAction => {
                self.graph
                    .insert(subject, rdf::type_, cat::FiltrateAction)?;
            }
        }

        Ok(())
    }

    fn insert_an_action(&mut self, subject: &SimpleTerm, action: &Action) -> Result<()> {
        let action_term: SimpleTerm = generate_bnode_term();

        self.graph.insert(&action_term, cat::hasBatch, subject)?;

        self.insert_a_date_time(
            &action_term,
            &allores::AFX_0000622,
            action.start_time.as_str(),
        )?;
        self.insert_a_date_time(
            &action_term,
            &allores::AFR_0002423,
            &action.ending_time.as_str(),
        )?;
        self.graph.insert(
            &action_term,
            allores::AFR_0001606,
            action.method_name.as_str(),
        )?;
        self.graph.insert(
            &action_term,
            allores::AFR_0001723,
            action.equipment_name.as_str(),
        )?;
        self.graph.insert(
            &action_term,
            cat::subEquipmentName,
            action.sub_equipment_name.as_str(),
        )?;

        if let Some(container_info) = &action.container_info {
            self.insert_container_properties(&action_term, &container_info)?;
        }

        if let Some(temperature_shaker) = &action.temperature_shaker {
            self.insert_an_observation(
                &action_term,
                &cat::temperatureShakerShape,
                temperature_shaker,
            )
            .context("Failed to insert observation")?
        }

        if let Some(temperature_tumble_stirrer) = &action.temperature_tumble_stirrer {
            self.insert_an_observation(
                &action_term,
                &cat::temperatureTumbleStirrerShape,
                temperature_tumble_stirrer,
            )
            .context("Failed to insert observation")?
        }

        if let Some(speed_shaker) = &action.speed_shaker {
            self.insert_an_observation(&action_term, &cat::speedInRPM, speed_shaker)?;
        }

        if let Some(dispense_type) = &action.dispense_type {
            self.graph
                .insert(&action_term, cat::dispenseType, dispense_type.as_str())?;
        }

        if let Some(dispense_state) = &action.dispense_state {
            self.graph
                .insert(&action_term, alloqual::AFQ_0000111, dispense_state.as_str())?;
        }

        if let Some(container_position_and_quantities) = &action.has_container_position_and_quantity {
            for container_position_quantity_item in container_position_and_quantities {
                self.insert_a_container_position_and_quantity(&action_term, container_position_quantity_item)?;
            }
        }

        if let Some(sample) = &action.has_sample {
            self.insert_samples(&action_term, sample)?;
        }

        self.insert_action_type(&action_term, action)?;

        Ok(())
    }

    /// Insert a batch
    ///
    /// Assumes a new graph has been created
    ///
    /// # Returns
    /// A `Result` containing `()` if successful, or an error if the graph building fails.
    pub fn insert_a_batch(&mut self, batch: &Batch) -> Result<()> {
        let batch_term = generate_bnode_term();

        self.graph.insert(&batch_term, rdf::type_, cat::Batch)?;
        self.graph
            .insert(&batch_term, schema::name, batch.batch_id.as_str())?;

        for action in &batch.actions {
            self.insert_an_action(&batch_term, action)?;
        }

        Ok(())
    }

    /// Get the turtle serialization of the RDF graph
    ///
    /// Assumes a new graph has been created and built.
    ///
    /// # Returns
    /// A `Result` containing the graph as Turtle serialization, or an error
    /// if the graph retrieval fails.
    pub fn serialize_to_turtle(&self) -> Result<String> {
        serialize_graph_to_turtle(&self.graph).context("Failed to serialize graph to Turtle")
    }

    /// Get the turtle serialization of the RDF graph
    ///
    /// Assumes a new graph has been created and built.
    ///
    /// # Returns
    ///  The `jsonld` serialization of the grap, or an error otherwise.
    /// if the graph retrieval fails.
    pub fn serialize_to_jsonld(&self) -> Result<String> {
        serialize_graph_to_jsonld(&self.graph).context("Failed to serialize graph to JSON-LD")
    }
}
