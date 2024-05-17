#![allow(non_upper_case_globals)]
use wasm_bindgen::prelude::*;

pub mod lightweight_charts {
    use super::*;

    #[wasm_bindgen]
    extern "C" {
        #[wasm_bindgen(js_namespace=LightweightCharts, js_name="TimeChartOptions")]
        pub type ITimeChartOptions;
        #[wasm_bindgen(js_namespace=LightweightCharts, js_name="PriceScaleOptions")]
        pub type IPriceScaleOptions;

        #[wasm_bindgen(js_namespace=LightweightCharts)]
        pub type IChartApi;

        #[wasm_bindgen(js_namespace=LightweightCharts)]
        pub fn createChart(container: JsValue, options: ChartOptions) -> IChartApi;
    }

    #[wasm_bindgen(getter_with_clone)]
    #[derive(Debug, Default, Clone)]
    pub struct PriceScaleOptions {
        #[wasm_bindgen(js_name = "autoScale")]
        pub auto_scale: bool,
        #[wasm_bindgen(js_name = "invertScale")]
        pub invert_scale: bool,
        #[wasm_bindgen(js_name = "alignLabels")]
        pub align_labels: bool,
        #[wasm_bindgen(js_name = "borderVisible")]
        pub border_visible: bool,
        #[wasm_bindgen(js_name = "borderColor")]
        pub border_color: String,
        #[wasm_bindgen(js_name = "textColor")]
        pub text_color: String,
        #[wasm_bindgen(js_name = "entireTextOnly")]
        pub entire_text_only: bool,
        pub visible: bool,
        #[wasm_bindgen(js_name = "ticksVisible")]
        pub ticks_visible: bool,
        #[wasm_bindgen(js_name = "minimumWidth")]
        pub minimum_width: u32,
    }

    #[derive(Debug, Default, Clone)]
    pub enum CrosshairMode {
        #[default]
        Normal = 0,
        Magnet = 1,
        Hidden = 2,
    }

    // #[wasm_bindgen(getter_with_clone)]
    // #[derive(Debug, Default, Clone)]
    // pub struct CrosshairLineOptions {
    //     #[wasm_bindgen(js_name="autoScale")]
    //     pub auto_scale: bool,
    //     #[wasm_bindgen(js_name="invertScale")]
    //     pub invert_scale: bool,
    //     #[wasm_bindgen(js_name="alignLabels")]
    //     pub align_labels: bool,
    //     #[wasm_bindgen(js_name="borderVisible")]
    //     pub border_visible: bool,
    //     pub color: String,
    //     pub width: ,
    //     #[wasm_bindgen(js_name="textColor")]
    //     pub text_color: String,
    //     #[wasm_bindgen(js_name="entireTextOnly")]
    //     pub entire_text_only: bool,
    //     pub visible: bool,
    //     #[wasm_bindgen(js_name="ticksVisible")]
    //     pub ticks_visible: bool,
    //     #[wasm_bindgen(js_name="minimumWidth")]
    //     pub minimum_width: u32,
    // }

    pub struct CrosshairOptions {
        pub mode: CrosshairMode,
    }

    #[wasm_bindgen(getter_with_clone)]
    #[derive(Debug, Default, Clone)]
    pub struct ChartOptions {
        pub width: u32,
        pub height: u32,
        #[wasm_bindgen(js_name = "autoSize")]
        pub auto_size: bool,
        #[wasm_bindgen(js_name = "rightPriceScale")]
        pub right_price_scale: PriceScaleOptions,
        #[wasm_bindgen(js_name = "leftPriceScale")]
        pub left_price_scale: PriceScaleOptions,
        #[wasm_bindgen(js_name = "overlayPriceScales")]
        pub overlay_price_scales: PriceScaleOptions,
        #[wasm_bindgen(js_name = "timeScale")]
        pub time_scale: PriceScaleOptions,
    }
}

pub(crate) mod advanced_charts {
    use super::*;
}

#[wasm_bindgen]
pub fn get_container(id: &str) -> web_sys::Element {
    let document = web_sys::window().unwrap().document().unwrap();
    document.get_element_by_id(id).unwrap()
}

#[wasm_bindgen]
extern "C" {

    #[wasm_bindgen(js_name=tvWidget)]
    pub static tv_widget: JsValue;

    #[wasm_bindgen(js_name=tvContainer)]
    pub static tv_chart_container: JsValue;

    #[wasm_bindgen(getter=initialize)]
    pub fn initialize();
}
