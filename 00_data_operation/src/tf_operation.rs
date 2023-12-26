/* Copyright 2023-2033 WanSen AI Team, Inc. All Rights Reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://opensource.wansenai.com/apache2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
==============================================================================*/

use tensorflow::{Scope as scope, Tensor};

fn tf_ops_range() {

    let mut scope = scope::new_root_scope();
    let x = tensorflow::ops::constant(&[0,  1,  2,  3,  4,  5,  6,  7,  8,  9, 10, 11][..], &mut scope).unwrap();
    println!("x type: {:?}", x.get_attr_type("dtype").unwrap());

    let value: Tensor<i32> = x.get_attr_tensor("value").unwrap();
    println!("x value: {:?}", value);

    let shape = x.get_attr_shape("value").unwrap();
    println!("x shape: {:?}",  shape);
}

// test tf_ops_range
#[test]
fn test_tf_ops_range() {
    tf_ops_range();
}