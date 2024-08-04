<?php

$rbm = new RoaringBitmap();
$rbm->insert(11);

echo count($rbm);
echo $rbm->ss();
// echo $rbm;

// $arr = new EvenNumbersArray();
// var_dump($arr[0]); // true
//  var_dump($arr[1]); // false
//  var_dump($arr[2]); // true
//  var_dump($arr[3]); // false
//  var_dump($arr[4]); // true
//  var_dump($arr[5] = true);