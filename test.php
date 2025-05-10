<?php

function assert_eq($a, $b, $msg = '') {
    if ($a !== $b) {
        echo "Assertion failed: ", var_export($a, true), " !== ", var_export($b, true), " $msg\n";
        debug_print_backtrace(DEBUG_BACKTRACE_IGNORE_ARGS, 1);
        exit(1);
    }
}

function assert_true($cond, $msg = '') {
    if (!$cond) {
        echo "Assertion failed: condition is not true $msg\n";
        debug_print_backtrace(DEBUG_BACKTRACE_IGNORE_ARGS, 1);
        exit(1);
    }
}

function assert_false($cond, $msg = '') {
    if ($cond) {
        echo "Assertion failed: condition is not false $msg\n";
        debug_print_backtrace(DEBUG_BACKTRACE_IGNORE_ARGS, 1);
        exit(1);
    }
}

// Test insert, contains, remove
$rbm = new RoaringBitmap();
assert_false($rbm->contains(42), 'should not contain 42');
$rbm->insert(42);
assert_true($rbm->contains(42), 'should contain 42 after insert');
$rbm->remove(42);
assert_false($rbm->contains(42), 'should not contain 42 after remove');

// Test count, isEmpty
$rbm = new RoaringBitmap();
assert_eq($rbm->count(), 0, 'new bitmap count');
assert_true($rbm->isEmpty(), 'new bitmap is empty');
$rbm->insert(1);
assert_eq($rbm->count(), 1, 'count after insert');
assert_false($rbm->isEmpty(), 'not empty after insert');

// Test min, max
$rbm = new RoaringBitmap();
assert_eq($rbm->min(), null, 'empty min');
assert_eq($rbm->max(), null, 'empty max');
$rbm->insert(10);
$rbm->insert(20);
$rbm->insert(5);
assert_eq($rbm->min(), 5, 'min after inserts');
assert_eq($rbm->max(), 20, 'max after inserts');

// Test union
$rbm1 = new RoaringBitmap();
$rbm2 = new RoaringBitmap();
foreach ([1,2,3] as $v) $rbm1->insert($v);
foreach ([3,4,5] as $v) $rbm2->insert($v);
$rbm1->union($rbm2);
$result = [];
for ($i = 1; $i <= 5; $i++) if ($rbm1->contains($i)) $result[] = $i;
assert_eq($result, [1,2,3,4,5], 'union result');

// Test intersect
$rbm1 = new RoaringBitmap();
$rbm2 = new RoaringBitmap();
foreach ([1,2,3] as $v) $rbm1->insert($v);
foreach ([2,3,4] as $v) $rbm2->insert($v);
$rbm1->intersect($rbm2);
$result = [];
for ($i = 1; $i <= 4; $i++) if ($rbm1->contains($i)) $result[] = $i;
assert_eq($result, [2,3], 'intersect result');

// Test difference
$rbm1 = new RoaringBitmap();
$rbm2 = new RoaringBitmap();
foreach ([1,2,3] as $v) $rbm1->insert($v);
foreach ([2,3,4] as $v) $rbm2->insert($v);
$rbm1->difference($rbm2);
$result = [];
for ($i = 1; $i <= 4; $i++) if ($rbm1->contains($i)) $result[] = $i;
assert_eq($result, [1], 'difference result');

// Test symmetricDifference
$rbm1 = new RoaringBitmap();
$rbm2 = new RoaringBitmap();
foreach ([1,2,3] as $v) $rbm1->insert($v);
foreach ([2,3,4] as $v) $rbm2->insert($v);
$rbm1->symmetricDifference($rbm2);
$result = [];
for ($i = 1; $i <= 4; $i++) if ($rbm1->contains($i)) $result[] = $i;
assert_eq($result, [1,4], 'symmetricDifference result');

// Test isSubset, isSuperset, isDisjoint
$rbm1 = new RoaringBitmap();
$rbm2 = new RoaringBitmap();
$rbm3 = new RoaringBitmap();
foreach ([1,2] as $v) $rbm1->insert($v);
foreach ([1,2,3] as $v) $rbm2->insert($v);
foreach ([4,5] as $v) $rbm3->insert($v);
assert_true($rbm1->isSubset($rbm2), 'isSubset');
assert_true($rbm2->isSuperset($rbm1), 'isSuperset');
assert_true($rbm1->isDisjoint($rbm3), 'isDisjoint');

echo "All tests passed.\n";