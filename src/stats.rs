#![allow(dead_code)]
pub mod base{
    use std::{
        collections::HashMap, vec,
        ops::{Add, AddAssign}
    };

    // pub fn sum<T:Add<Output = T>>(a: T, b:T) -> T {
    //     a + b
    // }

    // pub fn meanv2<T:Add<Output = T>>(v:&Vec<T>) -> T 
    //     where for<'a> &T: Add<&'a T, Output = <&'a T>{
    //     let mut sum:&T = &v[0];

    //     for n in 0..v.len() {
    //         sum = sum + v[n];
    //     }
        
    //     sum
    // }

    pub fn mean(v:&Vec<f32>) -> f32{
        let mut sum:f32 = 0.0;
        for num in v{
            sum += num;
        }
        return sum / (v.len() as f32);
    }

    pub fn median(v:&Vec<f32>) -> f32{
        let len:usize = v.len();

        if len % 2 == 0{
            return (v[len/2] + v[len/2 - 1]) / 2.0;
        }
        return v[len/2];
    }

    //to do
    pub fn mode(v:&Vec<i32>){
        let mut quant_map:HashMap<i32, usize> = HashMap::new();

        for num in v{
            if !quant_map.contains_key(num){
                quant_map.insert(*num , 1);
            }else {
                *quant_map.get_mut(num).unwrap() += 1;
            }
        }
        
    }

    pub fn variance(v:&Vec<f32>) -> f32{
        let mean:f32 = mean(&v);
        let mut sum:f32 = 0.0;

        for n in v{
            let sample:f32 = (*n - mean).powf(2.0);
            sum += sample;
        }

        return sum / (v.len() as f32 - 1.0);
    }

    pub fn std_deviation(v:&Vec<f32>) -> f32{
        return variance(&v).sqrt();
    }

    pub fn coef_variation(v:&Vec<f32>) -> f32{
        return std_deviation(&v) / mean(&v);
    }

    pub fn covariance(v1:&Vec<f32>, v2:&Vec<f32>) -> f32{
        if v1.len() != v2.len(){return 0.0;}
        let m1:f32 = mean(&v1);
        let m2:f32 = mean(&v2);

        let mut sum:f32 = 0.0;

        let mut count:usize = 0;
        while count < v1.len() {
            sum += (v1[count] - m1) * (v2[count] - m2);

            count += 1;
        }        

        return sum / (v1.len() as f32 - 1.0);
    }

    pub fn correlation_coef(v1:&Vec<f32>, v2:&Vec<f32>) -> f32{
        let std_dev1:f32 = std_deviation(&v1);
        let std_dev2:f32 = std_deviation(&v2);

        return covariance(v1, v2) / (std_dev1 * std_dev2);
    }

    pub fn normalize_list(v:&Vec<f32>) -> Vec<f32>{
        let mean:f32 = mean(&v);
        let std_dev:f32 = std_deviation(&v);

        let mut normalized_v:Vec<f32> = vec![];

        for n in v{
            let norm_n:f32 = (n - mean) / std_dev;
            normalized_v.push(norm_n);
        }

        return normalized_v;
    }

    pub fn sample_error(v:&Vec<f32>) -> f32{
        let std_dev:f32 = std_deviation(&v);

        return std_dev / ((v.len() as f32).sqrt())
    }

    fn crit_prob(confidence:f32) -> f32{
        let alpha:f32 = 1.0 - confidence;
        return 1.0 - (alpha/ 2.0);
    }

    pub fn z_score(n:f32, mean:f32, std_dev:f32) -> f32{
        return (n-mean)/std_dev;
    }

    pub fn get_z_value(confidence:f32) -> (Option<f32>, Option<f32>){
        if confidence < 0.0 || confidence > 1.0{ return (None, None); }

        let z_scores_table:Vec<Vec<f32>> = vec![
            vec![0.50000,0.50399,0.50798,0.51197,0.51595,0.51994,0.52392,0.52790,0.53188,0.53586],
            vec![0.53983,0.54380,0.54776,0.55172,0.55567,0.55962,0.56360,0.56749,0.57142,0.57535],
            vec![0.57926,0.58317,0.58706,0.59095,0.59483,0.59871,0.60257,0.60642,0.61026,0.61409],
            vec![0.61791,0.62172,0.62552,0.62930,0.63307,0.63683,0.64058,0.64431,0.64803,0.65173],
            vec![0.65542,0.65910,0.66276,0.66640,0.67003,0.67364,0.67724,0.68082,0.68439,0.68793],
            vec![0.69146,0.69497,0.69847,0.70194,0.70540,0.70884,0.71226,0.71566,0.71904,0.72240],
            vec![0.72575,0.72907,0.73237,0.73565,0.73891,0.74215,0.74537,0.74857,0.75175,0.75490],
            vec![0.75804,0.76115,0.76424,0.76730,0.77035,0.77337,0.77637,0.77935,0.78230,0.78524],
            vec![0.78814,0.79103,0.79389,0.79673,0.79955,0.80234,0.80511,0.80785,0.81057,0.81327],
            vec![0.81594,0.81859,0.82121,0.82381,0.82639,0.82894,0.83147,0.83398,0.83646,0.83891],//10
            vec![0.84134,0.84375,0.84614,0.84849,0.85083,0.85314,0.85543,0.85769,0.85993,0.86214],
            vec![0.86433,0.86650,0.86864,0.87076,0.87286,0.87493,0.87698,0.87900,0.88100,0.88298],
            vec![0.88493,0.88686,0.88877,0.89065,0.89251,0.89435,0.89617,0.89796,0.89973,0.90147],
            vec![0.90320,0.90490,0.90658,0.90824,0.90988,0.91149,0.91308,0.91466,0.91621,0.91774],
            vec![0.91924,0.92073,0.92220,0.92364,0.92507,0.92647,0.92785,0.92922,0.93056,0.93189],
            vec![0.93319,0.93448,0.93574,0.93699,0.93822,0.93943,0.94062,0.94179,0.94295,0.94408],
            vec![0.94520,0.94630,0.94738,0.94845,0.94950,0.95053,0.95154,0.95254,0.95352,0.95449],
            vec![0.95543,0.95637,0.95728,0.95818,0.95907,0.95994,0.96080,0.96164,0.96246,0.96327],
            vec![0.96407,0.96485,0.96562,0.96638,0.96712,0.96784,0.96856,0.96926,0.96995,0.97062],
            vec![0.97128,0.97193,0.97257,0.97320,0.97381,0.97441,0.97500,0.97558,0.97615,0.97670],//20
            vec![0.97725,0.97778,0.97831,0.97882,0.97932,0.97982,0.98030,0.98077,0.98124,0.98169],
            vec![0.98214,0.98257,0.98300,0.98341,0.98382,0.98422,0.98461,0.98500,0.98537,0.98574],
            vec![0.98610,0.98645,0.98679,0.98713,0.98745,0.98778,0.98809,0.98840,0.98870,0.98899],
            vec![0.98928,0.98956,0.98983,0.99010,0.99036,0.99061,0.99086,0.99111,0.99134,0.99158],
            vec![0.99180,0.99202,0.99224,0.99245,0.99266,0.99286,0.99305,0.99324,0.99343,0.99361],
            vec![0.99379,0.99396,0.99413,0.99430,0.99446,0.99461,0.99477,0.99492,0.99506,0.99520],
            vec![0.99534,0.99547,0.99560,0.99573,0.99585,0.99598,0.99609,0.99621,0.99632,0.99643],
            vec![0.99653,0.99664,0.99674,0.99683,0.99693,0.99702,0.99711,0.99720,0.99728,0.99736],
            vec![0.99744,0.99752,0.99760,0.99767,0.99774,0.99781,0.99788,0.99795,0.99801,0.99807],
            vec![0.99813,0.99819,0.99825,0.99831,0.99836,0.99841,0.99846,0.99851,0.99856,0.99861],//30
            vec![0.99865,0.99869,0.99874,0.99878,0.99882,0.99886,0.99889,0.99893,0.99896,0.99900],
            vec![0.99903,0.99906,0.99910,0.99913,0.99916,0.99918,0.99921,0.99924,0.99926,0.99929],
            vec![0.99931,0.99934,0.99936,0.99938,0.99940,0.99942,0.99944,0.99946,0.99948,0.99950],
            vec![0.99952,0.99953,0.99955,0.99957,0.99958,0.99960,0.99961,0.99962,0.99964,0.99965],
            vec![0.99966,0.99968,0.99969,0.99970,0.99971,0.99972,0.99973,0.99974,0.99975,0.99976],
            vec![0.99977,0.99978,0.99978,0.99979,0.99980,0.99981,0.99981,0.99982,0.99983,0.99983],
            vec![0.99984,0.99985,0.99985,0.99986,0.99986,0.99987,0.99987,0.99988,0.99988,0.99989],
            vec![0.99989,0.99990,0.99990,0.99990,0.99991,0.99991,0.99992,0.99992,0.99992,0.99992],
            vec![0.99993,0.99993,0.99993,0.99994,0.99994,0.99994,0.99994,0.99995,0.99995,0.99995],
            vec![0.99995,0.99995,0.99996,0.99996,0.99996,0.99996,0.99996,0.99996,0.99997,0.99997],
            vec![0.99997,0.99997,0.99997,0.99997,0.99997,0.99997,0.99998,0.99998,0.99998,0.99998],
        ];

        let mut zi:usize = 0;
        let mut zj:usize = 0;
        let crit:f32 = crit_prob(confidence);

        'outer: for i in &z_scores_table{
            for num in i{
                if num >= &crit {
                    // println!("Found [{}][{}] {}",zi, zj, num);
                    break 'outer;
                }
                zj+=1;
            }
            zi += 1;
            zj = 0;
        }

        let z_tab_value:f32 = z_scores_table[zi][zj];

        let pos:String = format!("{}", zi);
        let mut first:String = String::new();
        let second:String = format!("{}", zj);
        if pos.len() < 2{
            first.push_str("0.");
            first.push_str(&pos);
            first.push_str(&second);
        }else {
            let chars:Vec<char> = pos.chars().collect();
            first.push(chars[0]);
            first.push('.');
            first.push(chars[1]);
            first.push_str(&second);
        }

        let code:f32 = first.parse().unwrap();

        return (Some(code), Some(z_tab_value));
    }

    pub fn z_confidence_interval(v:&Vec<f32>, confidence:f32){
        let mean:f32 = mean(&v);
        let std_dev:f32 = std_deviation(&v);
        let size:f32 = v.len() as f32;

        let alpha:f32 = 1.0 - confidence;
        let crit_prob:f32 = crit_prob(confidence);

        let mut z_code:f32 = 0.0;
        if let (Some(code), Some(_tab_value)) = get_z_value(confidence){ 
            z_code = code; 
        }

        let error_margin:f64 = (z_code * (std_dev / size.sqrt())) as f64;
        let intrval_n:f64 = mean as f64 - error_margin;
        let intrval_p:f64 = mean as f64 + error_margin;

        println!("    Z Distribution");
        println!("    Alpha:[{}]", alpha);
        println!("    Crit.Prob:[{}]", crit_prob);
        println!("    Z-code:[{}] = {} ", crit_prob, z_code);
        println!("    Margin:[{:.2}] = [ {:.2} : {:.2} ]", error_margin, intrval_n, intrval_p);

    }

    pub fn get_t_value(v:&Vec<f32>, confidence:f32) -> Option<f32>{
        if confidence < 0.0 || confidence > 1.0{ return None; }

        let t_table:Vec<Vec<f32>> = vec![
            vec![1.000, 1.376,  1.963,  3.078,  6.314,  12.706,  31.821, 63.657, 127.321,318.309,636.619],
            vec![0.816, 1.061,  1.386,  1.886,  2.920,  4.303,   6.965,  9.925,  14.089, 22.327, 31.599],
            vec![0.765, 0.978,  1.250,  1.638,  2.353,  3.182,   4.541,  5.841,  7.453,  10.215, 12.924],
            vec![0.741, 0.941,  1.190,  1.533,  2.132,  2.776,   3.747,  4.604,  5.598,  7.173,  8.610],
            vec![0.727, 0.920,  1.156,  1.476,  2.015,  2.571,   3.365,  4.032,  4.773,  5.893,  6.869],
            vec![0.718, 0.906,  1.134,  1.440,  1.943,  2.447,   3.143,  3.707,  4.317,  5.208,  5.959],
            vec![0.711, 0.896,  1.119,  1.415,  1.895,  2.365,   2.998,  3.499,  4.029,  4.785,  5.408],
            vec![0.706, 0.889,  1.108,  1.397,  1.860,  2.306,   2.896,  3.355,  3.833,  4.501,  5.041],
            vec![0.703, 0.883,  1.100,  1.383,  1.833,  2.262,   2.821,  3.250,  3.690,  4.297,  4.781],
            vec![0.700, 0.879,  1.093,  1.372,  1.812,  2.228,   2.764,  3.169,  3.581,  4.144,  4.587],//10
            vec![0.697, 0.876,  1.088,  1.363,  1.796,  2.201,   2.718,  3.106,  3.497,  4.025,  4.437],
            vec![0.695, 0.873,  1.083,  1.356,  1.782,  2.179,   2.681,  3.055,  3.428,  3.930,  4.318],
            vec![0.694, 0.870,  1.079,  1.350,  1.771,  2.160,   2.650,  3.012,  3.372, 3.852, 	4.221],
            vec![0.692, 0.868,  1.076,  1.345,  1.761,  2.145, 	 2.624,  2.977,  3.326, 3.787, 	4.140],
            vec![0.691, 0.866,  1.074,  1.341,  1.753,  2.131, 	 2.602,  2.947,  3.286, 3.733, 	4.073],
            vec![0.690, 0.865,  1.071,  1.337,  1.746,  2.120, 	 2.583,  2.921,  3.252, 3.686, 	4.015],
            vec![0.689, 0.863,  1.069,  1.333,  1.740,  2.110, 	 2.567,  2.898,  3.222, 3.646, 	3.965],
            vec![0.688, 0.862,  1.067,  1.330,  1.734,  2.101, 	 2.552,  2.878,  3.197, 3.610, 	3.922],
            vec![0.688, 0.861,  1.066,  1.328,  1.729,  2.093, 	 2.539,  2.861,  3.174, 3.579, 	3.883],
            vec![0.687, 0.860,  1.064,  1.325,  1.725,  2.086, 	 2.528,  2.845,  3.153, 3.552, 	3.850],//20
            vec![0.686, 0.859,  1.063,  1.323,  1.721,  2.080, 	 2.518,  2.831,  3.135, 3.527, 	3.819],
            vec![0.686, 0.858,  1.061,  1.321,  1.717,  2.074, 	 2.508,  2.819,  3.119, 3.505, 	3.792],
            vec![0.685, 0.858,  1.060,  1.319,  1.714,  2.069, 	 2.500,  2.807,  3.104, 3.485, 	3.767],
            vec![0.685, 0.857,  1.059,  1.318,  1.711,  2.064, 	 2.492,  2.797,  3.091, 3.467, 	3.745],
            vec![0.684, 0.856,  1.058,  1.316,  1.708,  2.060, 	 2.485,  2.787,  3.078, 3.450, 	3.725],
            vec![0.684, 0.856,  1.058,  1.315,  1.706,  2.056, 	 2.479,  2.779,  3.067, 3.435, 	3.707],
            vec![0.684, 0.855,  1.057,  1.314,  1.703,  2.052, 	 2.473,  2.771,  3.057, 3.421, 	3.690],
            vec![0.683, 0.855,  1.056,  1.313,  1.701,  2.048, 	 2.467,  2.763,  3.047, 3.408, 	3.674],
            vec![0.683,	0.854,	1.055,	1.310,	1.697,	2.042,	 2.457,	 2.750,	 3.030,	3.385,	3.646],
            vec![0.683, 0.854,  1.055,  1.311,  1.699,  2.045, 	 2.462,  2.756,  3.038, 3.396, 	3.659],//30
            vec![0.681, 0.851,  1.050,  1.303,  1.684,  2.021, 	 2.423,  2.704,  2.971, 3.307, 	3.551],//40
            vec![0.679, 0.849,  1.047,  1.299,  1.676,  2.009, 	 2.403,  2.678,  2.937, 3.261, 	3.496],//50
            vec![0.679, 0.848,  1.045,  1.296,  1.671,  2.000, 	 2.390,  2.660,  2.915, 3.232, 	3.460],//60
            vec![0.678, 0.846,  1.043,  1.292,  1.664,  1.990, 	 2.374,  2.639,  2.887, 3.195, 	3.416],//80
            vec![0.677, 0.845,  1.042,  1.290,  1.660,  1.984, 	 2.364,  2.626,  2.871, 3.174, 	3.390],//100
            vec![0.677, 0.845,  1.041,  1.289,  1.658,  1.980, 	 2.358,  2.617,  2.860, 3.160, 	3.373],//120
            vec![0.674, 0.842,  1.036,  1.282,  1.645,  1.960, 	 2.326,  2.576,  2.807, 3.090, 	3.291],//inf
        ];

        let t_table_confidence:Vec<f32> = 
            vec![0.25,  0.20,   0.15,   0.10,   0.05,   0.025,  0.01,   0.005,  0.0025, 0.001,  0.0005];

        let t_cols:Vec<u8> = 
            vec![1,2,3,4,5,6,7,8,9,10,11,12,13,14,15,16,17,18,19,20,21,22,23,24,25,26,27,28,29,30,40,50,60,80,100,120];

        let mut half_alpha:f32 = (1.0 - confidence) / 2.0;
        half_alpha = (half_alpha * 10000.0).round() / 10000.0;

        let deg_freedom:usize = v.len() - 2; 

        let mut i:usize = 0;
        while deg_freedom >= t_cols[i] as usize{ i += 1; }

        let mut j:usize = 0;
        while half_alpha <= t_table_confidence[j]{ j += 1; }

        return Some(t_table[i][j-1]);
    }

    pub fn t_confidence_interval(v:&Vec<f32>, confidence:f32){
        let size:f32 = v.len() as f32;
        let mean:f32 = mean(&v);
        let std_dev:f32 = std_deviation(&v);

        let mut t_value:f32 = 0.0;
        if let Some(t_val) = get_t_value(&v, confidence){
            t_value = t_val;
        }

        let error_margin:f64 = (t_value * (std_dev / (size - 2.0).sqrt())) as f64;
        let intrval_n:f64 = mean as f64 - error_margin;
        let intrval_p:f64 = mean as f64 + error_margin;

        println!("    T Distribution");
        println!("    Mean:[{}]", mean);
        println!("    Std-Dev:[{}]", std_dev);
        println!("    TVal:[{}]", t_value);
        println!("    Margin:[{:.2}] = [ {:.2} : {:.2} ]", error_margin, intrval_n, intrval_p);


    }

    //needs tuning, working only for z
    //p-val < z_val => reject Ho => accept Ha => true
    //p-val >= z_val => fail to reject Ho true => false
    pub fn p_val_test(mut z_value:f32, significance:f32) -> bool{
        z_value = 1.0 - z_value;
        if significance >= z_value {
            println!("    p-val:{:.4} >= z:{:.4} => failde to reject Ho", significance, z_value);
            return false;
        }else {
            println!("    p-val:{:.4} < z:{:.4} => Ho rejected", significance, z_value);
            return true;
        }
    }

    //to do
    pub fn get_linear_regression_list(v:&Vec<Vec<f32>>){
        let mut x_sum:f32 = 0.0;
        let mut y_sum:f32 = 0.0;

        let size:usize = v.len();
        for i in 0..size{
            for j in 0.. v[i].len(){
                x_sum += v[i][j];
                y_sum += v[i][j];
            }
        }

        let x_bar:f32 = x_sum / v[0].len() as f32;
        let y_bar:f32 = y_sum / v[0].len() as f32;

        let mut numer:f32 = 0.0;
        let mut denom:f32 = 0.0;
        for i in 0..size{
            for j in 0..v[i].len(){
                let x_sums:f32 = v[i][j] - x_bar;
                denom += x_sums.powi(2);
                numer += x_sums * (v[i][j] - y_bar);
            }
        }

        let slope:f32 = numer / denom;
        let _y_intercept:f32 = y_bar - slope * x_bar;
    }

}