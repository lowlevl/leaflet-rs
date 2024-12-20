use std::collections::HashMap;

use js_sys::{Array, Object};
use wasm_bindgen::prelude::*;

use super::Control;
use crate::{create_object_with_properties, ControlOptions, Layer};

#[wasm_bindgen]
extern "C" {
    #[derive(Debug, Clone)]
    #[wasm_bindgen(js_namespace = ["L", "control"], js_name = Layers, extends = Control)]
    pub type LayerControl;

    #[wasm_bindgen(js_namespace = ["L", "control"], js_name = "layers")]
    fn contructor_layer_control(
        baselayers: Object,
        overlays: Object,
        options: Option<LayerControlOptions>,
    ) -> LayerControl;

    // Methods for Layers and Controls

    #[wasm_bindgen(method, js_name = addBaseLayer)]
    pub fn add_base_layer(this: &LayerControl, layer: &Layer, name: &str) -> LayerControl;

    #[wasm_bindgen(method, js_name = addOverlay)]
    pub fn add_overlay(this: &LayerControl, layer: &Layer, name: &str) -> LayerControl;

    #[wasm_bindgen(method, js_name = removeLayer)]
    pub fn remove_layer(this: &LayerControl, layer: &Layer) -> LayerControl;

    #[wasm_bindgen(method)]
    pub fn expand(this: &LayerControl) -> LayerControl;

    #[wasm_bindgen(method)]
    pub fn collapse(this: &LayerControl) -> LayerControl;
}

create_object_with_properties!(
    (LayerControlOptions, LayerControlOptions, ControlOptions),
    (collapsed, collapsed, bool),
    (auto_z_index, autoZIndex, bool),
    (hide_single_base, hideSingleBase, bool),
    (sort_layers, sortLayers, bool),
    (sort_function, sortFunction, JsValue)
);

impl Default for LayerControlOptions {
    fn default() -> Self {
        LayerControlOptions::new()
    }
}

impl LayerControl {
    /// Creates a new [`LayerControl`] instance.
    #[must_use]
    pub fn new(baselayers: HashMap<&str, &Layer>, overlays: HashMap<&str, &Layer>) -> Self {
        let baselayers = baselayers
            .into_iter()
            .map(|(k, v)| Array::of2(&k.into(), v))
            .collect::<Array>();
        let overlays = overlays
            .into_iter()
            .map(|(k, v)| Array::of2(&k.into(), v))
            .collect::<Array>();

        contructor_layer_control(
            Object::from_entries(&baselayers).unwrap(),
            Object::from_entries(&overlays).unwrap(),
            None,
        )
    }

    /// Creates a new [`LayerControl`] instance, with options.
    #[must_use]
    pub fn new_with_options(
        baselayers: HashMap<&str, &Layer>,
        overlays: HashMap<&str, &Layer>,
        options: LayerControlOptions,
    ) -> Self {
        let baselayers = baselayers
            .into_iter()
            .map(|(k, v)| Array::of2(&k.into(), v))
            .collect::<Array>();
        let overlays = overlays
            .into_iter()
            .map(|(k, v)| Array::of2(&k.into(), v))
            .collect::<Array>();

        contructor_layer_control(
            Object::from_entries(&baselayers).unwrap(),
            Object::from_entries(&overlays).unwrap(),
            Some(options),
        )
    }
}
