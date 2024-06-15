use rust_bert::gpt_neo::{
    GptNeoConfig, GptNeoConfigResources, GptNeoMergesResources, GptNeoModelResources,
    GptNeoVocabResources,
};
use rust_bert::pipelines::common::ModelType;
use rust_bert::pipelines::sentence_embeddings::builder::Remote;
use rust_bert::pipelines::text_generation::{TextGenerationConfig, TextGenerationModel};
use rust_bert::resources::RemoteResource;

fn main() {
    let model_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoModelResources::GPT_NEO_2_7B,
    ));

    let config_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoConfigResources::GPT_NEO_2_7B,
    ));

    let vocab_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoVocabResources::GPT_NEO_2_7B,
    ));

    let merges_resource = Box::new(RemoteResource::from_pretrained(
        GptNeoMergesResources::GPT_NEO_2_7B,
    ));

    let generate_config = TextGenerationConfig {
        model_type: ModelType::GPTNeo,
        model_resource,
        config_resource,
        vocab_resource,
        merges_resource,
        num_beams: 5,
        no_repeat_ngram_size: 2,
        max_length: 100,
        ..Default::default()
    };

    let model = TextGenerationModel::new(generate_config).unwrap();

    loop {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).unwrap();
        let split = line.split('/').collect::<Vec<&str>>();
        let slice = split.as_slice();
        let output = model.generate(&slice[1..], Some(slc[0]));

        for sentence in output {
            println!("{}", sentence);
        }
    }
}
