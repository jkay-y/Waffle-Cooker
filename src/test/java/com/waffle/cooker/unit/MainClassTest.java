package com.waffle.cooker.unit;

import org.junit.jupiter.api.Assertions;
import org.junit.jupiter.api.Disabled;
import org.junit.jupiter.api.Test;

public class MainClassTest {

    @Test
    void test_sample() {
        //
        Assertions.assertEquals(4, 2 + 2);
    }

    @Disabled
    void test_sample2() {
        //
        Assertions.assertNotNull(null);
    }

}
