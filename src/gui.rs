use crate::biquad::FilterType;
use nannou::ui::prelude::*;

widget_ids! {
    pub struct Ids {
        fm_text,
        master_volume,
        retrigger,

        op1_text,
        op1_freq,
        op1_ratio,
        op1_ratio_offset,
        op1_attack,
        op1_decay,
        op1_sustain,
        op1_release,
        op1_amp,

        op2_text,
        op2_freq,
        op2_ratio,
        op2_ratio_offset,
        op2_attack,
        op2_decay,
        op2_sustain,
        op2_release,
        op2_amp,

        filter_text,
        filter_type,
        filter_cutoff,
        filter_resonance,
        filter_peak_gain,
    }
}

pub fn update(
    ref mut ui: UiCell,
    ids: &mut Ids,
    parameters: &mut crate::Parameters,
    producers: &mut crate::synth::Producers,
) {
    fn slider(val: f32, min: f32, max: f32) -> widget::Slider<'static, f32> {
        widget::Slider::new(val, min, max)
            .w_h(200.0, 30.0)
            .label_font_size(15)
            .rgb(0.3, 0.3, 0.3)
            .label_rgb(1.0, 1.0, 1.0)
            .border(0.0)
    }

    widget::Text::new("FM SYNTH")
        .top_left_with_margin(20.0)
        .color(color::WHITE)
        .font_size(16)
        .set(ids.fm_text, ui);

    let label = format!("Master Volume: {:.2}", parameters.master_volume);
    for value in slider(parameters.master_volume, 0.0, 1.0)
        .down(4.0)
        .label(&label)
        .set(ids.master_volume, ui)
    {
        parameters.master_volume = value;
    }

    for value in widget::Toggle::new(parameters.note_on_off)
        .down(4.0)
        .label("RETRIGGER")
        .w_h(200.0, 30.0)
        .label_font_size(15)
        .rgb(0.3, 0.3, 0.3)
        .label_rgb(1.0, 1.0, 1.0)
        .set(ids.retrigger, ui)
    {
        if producers.mod_env_on_off_producer.push(value).is_ok()
            && producers.carrier_env_on_off_producer.push(value).is_ok()
        {
            parameters.note_on_off = value;
        }
    }

    widget::Text::new("OPERATOR 1")
        .down(20.0)
        .color(color::WHITE)
        .font_size(16)
        .set(ids.op1_text, ui);

    let label = format!("Freq: {:.2}", parameters.op1.pitch.freq);
    for value in slider(parameters.op1.pitch.freq, 0.0, 12543.855)
        .down(4.0)
        .label(&label)
        .set(ids.op1_freq, ui)
    {
        if producers.mod_hz_producer.push(value as f64).is_ok() {
            parameters.op1.pitch.freq = value;
        }
    }

    let label = format!("Ratio: {:.2}", parameters.op1.pitch.ratio);
    for value in slider(parameters.op1.pitch.ratio, 0.0, 35.0)
        .down(4.0)
        .label(&label)
        .set(ids.op1_ratio, ui)
    {
        let idx = value.ceil() as usize;
        parameters.op1.pitch.ratio = crate::synth::MODULATOR_RATIOS[idx];
        crate::update_frequency(
            parameters.master_frequency,
            &mut parameters.op1,
            &mut producers.mod_hz_producer,
        );
    }

    let label = format!("Ratio Offset: {:.2}", parameters.op1.pitch.ratio_offset);
    for value in slider(parameters.op1.pitch.ratio_offset, -1.0, 1.0)
        .down(4.0)
        .label(&label)
        .set(ids.op1_ratio_offset, ui)
    {
        parameters.op1.pitch.ratio_offset = value;
        crate::update_frequency(
            parameters.master_frequency,
            &mut parameters.op1,
            &mut producers.mod_hz_producer,
        );
    }

    let label = format!("Attack: {:.2}", parameters.op1.env.attack);
    for value in slider(parameters.op1.env.attack, 0.0, 8.0)
        .down(4.0)
        .label(&label)
        .set(ids.op1_attack, ui)
    {
        if producers.mod_attack_producer.push(value as f64).is_ok() {
            parameters.op1.env.attack = value;
        }
    }

    let label = format!("Decay: {:.2}", parameters.op1.env.decay);
    for value in slider(parameters.op1.env.decay, 0.0, 1.0)
        .down(4.0)
        .label(&label)
        .set(ids.op1_decay, ui)
    {
        if producers.mod_decay_producer.push(value as f64).is_ok() {
            parameters.op1.env.decay = value;
        }
    }

    let label = format!("Sustain: {:.2}", parameters.op1.env.sustain);
    for value in slider(parameters.op1.env.sustain, 0.0, 1.0)
        .down(4.0)
        .label(&label)
        .set(ids.op1_sustain, ui)
    {
        if producers.mod_sustain_producer.push(value as f64).is_ok() {
            parameters.op1.env.sustain = value;
        }
    }

    let label = format!("Release: {:.2}", parameters.op1.env.release);
    for value in slider(parameters.op1.env.release, 0.0, 6.0)
        .down(4.0)
        .label(&label)
        .set(ids.op1_release, ui)
    {
        if producers.mod_release_producer.push(value as f64).is_ok() {
            parameters.op1.env.release = value;
        }
    }

    let label = format!("Amp: {:.2}", parameters.op1.amp);
    for value in slider(parameters.op1.amp, 0.0, 1000.0)
        .down(4.0)
        .label(&label)
        .set(ids.op1_amp, ui)
    {
        if producers.mod_amp_producer.push(value as f64).is_ok() {
            parameters.op1.amp = value;
        }
    }

    widget::Text::new("OPERATOR 2")
        .down(20.0)
        .color(color::WHITE)
        .font_size(16)
        .set(ids.op2_text, ui);

    let label = format!("Freq: {:.2}", parameters.op2.pitch.freq);
    for value in slider(parameters.op2.pitch.freq, 0.0, 12543.855)
        .down(4.0)
        .label(&label)
        .set(ids.op2_freq, ui)
    {
        if producers.carrier_hz_producer.push(value as f64).is_ok() {
            parameters.op2.pitch.freq = value;
        }
    }

    let label = format!("Ratio: {:.2}", parameters.op2.pitch.ratio);
    for value in slider(parameters.op2.pitch.ratio, 0.0, 18.0)
        .down(4.0)
        .label(&label)
        .set(ids.op2_ratio, ui)
    {
        let idx = value.ceil() as usize;
        parameters.op2.pitch.ratio = crate::synth::CARRIER_RATIOS[idx];
        crate::update_frequency(
            parameters.master_frequency,
            &mut parameters.op2,
            &mut producers.carrier_hz_producer,
        );
    }

    let label = format!("Ratio Offset: {:.2}", parameters.op2.pitch.ratio_offset);
    for value in slider(parameters.op2.pitch.ratio_offset, -1.0, 1.0)
        .down(4.0)
        .label(&label)
        .set(ids.op2_ratio_offset, ui)
    {
        parameters.op2.pitch.ratio_offset = value;
        crate::update_frequency(
            parameters.master_frequency,
            &mut parameters.op2,
            &mut producers.carrier_hz_producer,
        );
    }

    let label = format!("Attack: {:.2}", parameters.op2.env.attack);
    for value in slider(parameters.op2.env.attack, 0.0, 2.0)
        .down(4.0)
        .label(&label)
        .set(ids.op2_attack, ui)
    {
        if producers.carrier_attack_producer.push(value as f64).is_ok() {
            parameters.op2.env.attack = value;
        }
    }

    let label = format!("Decay: {:.2}", parameters.op2.env.decay);
    for value in slider(parameters.op2.env.decay, 0.0, 2.0)
        .down(4.0)
        .label(&label)
        .set(ids.op2_decay, ui)
    {
        if producers.carrier_decay_producer.push(value as f64).is_ok() {
            parameters.op2.env.decay = value;
        }
    }

    let label = format!("Sustain: {:.2}", parameters.op2.env.sustain);
    for value in slider(parameters.op2.env.sustain, 0.0, 1.0)
        .down(4.0)
        .label(&label)
        .set(ids.op2_sustain, ui)
    {
        if producers
            .carrier_sustain_producer
            .push(value as f64)
            .is_ok()
        {
            parameters.op2.env.sustain = value;
        }
    }

    let label = format!("Release: {:.2}", parameters.op2.env.release);
    for value in slider(parameters.op2.env.release, 0.0, 6.0)
        .down(4.0)
        .label(&label)
        .set(ids.op2_release, ui)
    {
        if producers
            .carrier_release_producer
            .push(value as f64)
            .is_ok()
        {
            parameters.op2.env.release = value;
        }
    }

    let label = format!("Amp: {:.2}", parameters.op2.amp);
    for value in slider(parameters.op2.amp, 0.0, 1.0)
        .down(4.0)
        .label(&label)
        .set(ids.op2_amp, ui)
    {
        if producers.carrier_amp_producer.push(value as f64).is_ok() {
            parameters.op2.amp = value;
        }
    }

    widget::Text::new("Filter")
        .down(20.0)
        .color(color::WHITE)
        .font_size(16)
        .set(ids.filter_text, ui);

    let filters = vec![
        "LOW PASS".to_string(),
        "HIGH PASS".to_string(),
        "BAND PASS".to_string(),
        "NOTCH".to_string(),
        "PEAK".to_string(),
        "LOW SHELF".to_string(),
        "HIGH SHELF".to_string(),
    ];

    if let Some(selected_idx) =
        widget::DropDownList::new(&filters, Some(parameters.filter.filter_type))
            .down(4.0)
            .w_h(200.0, 30.0)
            .label("Filter Types")
            .label_font_size(15)
            .set(ids.filter_type, ui)
    {
        parameters.filter.filter_type = selected_idx;
        let filter_type = match selected_idx {
            0 => FilterType::Lowpass,
            1 => FilterType::Highpass,
            2 => FilterType::Bandpass,
            3 => FilterType::Notch,
            4 => FilterType::Peak,
            5 => FilterType::Lowshelf,
            6 => FilterType::Highshelf,
            _ => unreachable!(),
        };

        if producers.filter_type_producer.push(filter_type).is_ok() {
            parameters.filter.filter_type = selected_idx;
        }
    }

    let label = format!("Cutoff: {:.2}", parameters.filter.cutoff);
    for value in slider(parameters.filter.cutoff, 20.0, 1000.0)
        .down(4.0)
        .label(&label)
        .set(ids.filter_cutoff, ui)
    {
        if producers.cutoff_producer.push(value as f64).is_ok() {
            parameters.filter.cutoff = value;
        }
    }

    let label = format!("Resonance: {:.2}", parameters.filter.resonance);
    for value in slider(parameters.filter.resonance, 0.01, 10.0)
        .down(4.0)
        .label(&label)
        .set(ids.filter_resonance, ui)
    {
        if producers.resonance_producer.push(value as f64).is_ok() {
            parameters.filter.resonance = value;
        }
    }

    let label = format!("Peak Gain: {:.2}", parameters.filter.peak_gain);
    for value in slider(parameters.filter.peak_gain, -6.0, 6.0)
        .down(4.0)
        .label(&label)
        .set(ids.filter_peak_gain, ui)
    {
        if producers.peak_gain_producer.push(value as f64).is_ok() {
            parameters.filter.peak_gain = value;
        };
    }
}
