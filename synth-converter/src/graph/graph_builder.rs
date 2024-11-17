use std::collections::HashMap;
use sophia::api::graph::MutableGraph;
use sophia::api::ns::Namespace;
use sophia::api::serializer::TripleSerializer;
use sophia::inmem::graph::FastGraph;
use sophia_term::RcTerm;
use sophia_api::term::SimpleTerm;
use sophia_turtle::serializer::turtle::TurtleSerializer;
use crate::parser::batch::Batch;

pub struct GraphBuilder {
    graph: FastGraph,
    ex: Namespace<String>,
    allores: Namespace<String>,
    schema: Namespace<String>,
    cat: Namespace<String>,
    rdf: Namespace<String>,
    action_counter: HashMap<String, usize>,
}

impl GraphBuilder {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            action_counter: HashMap::new(), 
            graph: FastGraph::new(),
            ex: Namespace::<String>::new("http://example.org/".to_string())?,
            allores: Namespace::<String>::new("http://purl.allotrope.org/ontologies/result#".to_string())?,
            schema: Namespace::<String>::new("https://schema.org/".to_string())?,
            cat: Namespace::<String>::new("http://example.org/cat#".to_string())?,
            rdf: Namespace::<String>::new("http://www.w3.org/1999/02/22-rdf-syntax-ns#".to_string())?,         
        })
    }

    fn get_action_uri(&mut self, action_name: &str) -> String {
        // Increment the counter for the given action name
        let count = self.action_counter.entry(action_name.to_string()).or_insert(0);
        *count += 1;

        // Generate a unique URI
        format!("{}_{}", action_name, *count)
    }
    
    fn map_action_to_rdfs_class(&self, action_name: &str) -> SimpleTerm {{
        // Match the action name and fetch the corresponding class from either `self.cat` or `self.allores`.
        let mapped_class = match action_name {
            "add" => self.cat.get("AddAction"),           // Get "AddAction" from `self.cat`
            "set_temperature" => self.cat.get("setTemperatureAction"), // Get "setTemperatureAction" from `self.cat`
            _ => self.allores.get("AFRE_0000001"),       // Default to "AFRE_0000001" from `self.allores`
        };
        println!("{:?}", mapped_class);
    }
    
    
    

    pub fn add_batch(&mut self, batch: &Batch) -> Result<(), Box<dyn std::error::Error>> {
        // Fully resolve the batch URI before the loop
        let ex_namespace = self.ex.clone();

        // Resolve the batch URI
        let batch_uri = ex_namespace.get(&batch.batch_id)?.clone();
    
        self.graph.insert(
            &batch_uri,
            &self.allores.get("AFR_0001120")?,
            batch.batch_id.as_str(),
        )?;

        for action in &batch.actions {
            println!("Processing action: {:?}", action.name);
    
            // Generate a unique action URI
            let unique_action_name = self.get_action_uri(&action.name);
            let action_uri = self.ex.get(&unique_action_name)?.clone(); // Use `?` to unwrap, then clone
    
            self.graph.insert(
                &batch_uri,
                &self.allores.get("AFRE_0000001")?,
                &action_uri,
            )?;

            self.map_action_to_rdfs_class(&action.name);

            // Insert the RDF type triple (action_uri rdf:type action_rdfs_class)
            /*self.graph.insert(
                &action_uri,
                &self.rdf.get("type")?, // RDF type predicate
                &action_rdfs_class,
            )?;*/

            let action_predicates = vec![
                (Some(action.name.as_str()), self.schema.get("name")?),
                (
                    action.equipment_local_name.as_ref().map(|x| x.as_str()),
                    self.cat.get("localEquipmentName")?,
                ),
                (
                    action.container_barcode.as_ref().map(|x| x.as_str()),
                    self.cat.get("containerBarcode")?,
                ),
                (
                    action.dispense_type.as_ref().map(|x| x.as_str()),
                    self.cat.get("dispenseType")?,
                ),
            ];
    
            for (field, predicate) in action_predicates {
                if let Some(value) = field {
                    self.graph.insert(&action_uri, &predicate, value)?;
                }
            }
        }

        Ok(())
    }

    pub fn serialize_to_turtle(&self) -> Result<String, Box<dyn std::error::Error>> {
        use sophia::api::serializer::Stringifier; // Import the required trait
    
        let mut serializer = TurtleSerializer::new_stringifier();
        serializer.serialize_graph(&self.graph)?;
        Ok(serializer.as_str().to_string())
    }
    
}