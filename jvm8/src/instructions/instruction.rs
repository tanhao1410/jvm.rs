pub trait Instruction: Debug {
    /// 取操作数
    fn fetch_operands(&mut self, reader: &mut BytecodeReader) {
        //默认不取操作数
    }

    ///执行指令
    fn execute(&mut self, thread: Arc<RwLock<Thread>>);
}

/// 根据code码创建指令
pub fn new_instruction(op_code: u8) -> Box<dyn Instruction> {
    match op_code {
        00 => Box::new(NOP {}),
        01 => Box::new(ACONST_NULL {}),
        02 => Box::new(ICONST_M1 {}),
        03 => Box::new(ICONST_0 {}),
        04 => Box::new(ICONST_1 {}),
        05 => Box::new(ICONST_2 {}),
        06 => Box::new(ICONST_3 {}),
        07 => Box::new(ICONST_4 {}),
        08 => Box::new(ICONST_5 {}),
        09 => Box::new(LCONST_0 {}),
        10 => Box::new(LCONST_1 {}),
        11 => Box::new(FCONST_0 {}),
        12 => Box::new(FCONST_1 {}),
        13 => Box::new(FCONST_2 {}),
        14 => Box::new(DCONST_0 {}),
        15 => Box::new(DCONST_1 {}),
        16 => Box::new(BIPUSH::new()),
        17 => Box::new(SIPUSH::new()),

        18 => Box::new(LDC::new()),
        19 => Box::new(LDC_W::new()),
        20 => Box::new(LDC2_W::new()),

        21 => Box::new(ILOAD::new0()),
        22 => Box::new(LLOAD::new0()),
        23 => Box::new(FLOAD::new0()),
        24 => Box::new(DLOAD::new0()),
        25 => Box::new(ALOAD::new0()),
        26 => Box::new(ILOAD_0 {}),
        27 => Box::new(ILOAD_1 {}),
        28 => Box::new(ILOAD_2 {}),
        29 => Box::new(ILOAD_3 {}),
        30 => Box::new(LLOAD_0 {}),
        31 => Box::new(LLOAD_1 {}),
        32 => Box::new(LLOAD_2 {}),
        33 => Box::new(LLOAD_3 {}),
        34 => Box::new(FLOAD_0 {}),
        35 => Box::new(FLOAD_1 {}),
        36 => Box::new(FLOAD_2 {}),
        37 => Box::new(FLOAD_3 {}),
        38 => Box::new(DLOAD_0 {}),
        39 => Box::new(DLOAD_1 {}),
        40 => Box::new(DLOAD_2 {}),
        41 => Box::new(DLOAD_3 {}),
        42 => Box::new(ALOAD_0 {}),
        43 => Box::new(ALOAD_1 {}),
        44 => Box::new(ALOAD_2 {}),
        45 => Box::new(ALOAD_3 {}),
        46 => Box::new(IALOAD {}),
        47 => Box::new(LALOAD {}),
        48 => Box::new(FALOAD {}),
        49 => Box::new(DALOAD {}),
        50 => Box::new(AALOAD {}),
        51 => Box::new(BALOAD {}),
        52 => Box::new(CALOAD {}),
        53 => Box::new(SALOAD {}),
        54 => Box::new(ISTORE::new0()),
        55 => Box::new(LSTORE::new0()),
        56 => Box::new(FSTORE::new0()),
        57 => Box::new(DSTORE::new0()),
        58 => Box::new(ASTORE::new0()),
        59 => Box::new(ISTORE_0 {}),
        60 => Box::new(ISTORE_1 {}),
        61 => Box::new(ISTORE_2 {}),
        62 => Box::new(ISTORE_3 {}),
        63 => Box::new(LSTORE_0 {}),
        64 => Box::new(LSTORE_1 {}),
        65 => Box::new(LSTORE_2 {}),
        66 => Box::new(LSTORE_3 {}),
        67 => Box::new(FSTORE_0 {}),
        68 => Box::new(FSTORE_1 {}),
        69 => Box::new(FSTORE_2 {}),
        70 => Box::new(FSTORE_3 {}),
        71 => Box::new(DSTORE_0 {}),
        72 => Box::new(DSTORE_1 {}),
        73 => Box::new(DSTORE_2 {}),
        74 => Box::new(DSTORE_3 {}),
        75 => Box::new(ASTORE_0 {}),
        76 => Box::new(ASTORE_1 {}),
        77 => Box::new(ASTORE_2 {}),
        78 => Box::new(ASTORE_3 {}),
        79 => Box::new(IASTORE {}),
        80 => Box::new(LASTORE {}),
        81 => Box::new(FASTORE {}),
        82 => Box::new(DASTORE {}),
        83 => Box::new(AASTORE {}),
        84 => Box::new(BASTORE {}),
        85 => Box::new(CASTORE {}),
        86 => Box::new(SASTORE {}),
        87 => Box::new(POP {}),
        88 => Box::new(POP2 {}),
        89 => Box::new(DUP {}),
        90 => Box::new(DUP_X1 {}),
        91 => Box::new(DUP_X2 {}),
        92 => Box::new(DUP2 {}),
        93 => Box::new(DUP2_X1 {}),
        94 => Box::new(DUP2_X2 {}),
        95 => Box::new(SWAP {}),
        96 => Box::new(IADD {}),
        97 => Box::new(LADD {}),
        98 => Box::new(FADD {}),
        99 => Box::new(DADD {}),
        100 => Box::new(ISUB {}),
        101 => Box::new(LSUB {}),
        102 => Box::new(FSUB {}),
        103 => Box::new(DSUB {}),
        104 => Box::new(IMUL {}),
        105 => Box::new(LMUL {}),
        106 => Box::new(FMUL {}),
        107 => Box::new(DMUL {}),
        108 => Box::new(IDIV {}),
        109 => Box::new(LDIV {}),
        110 => Box::new(FDIV {}),
        111 => Box::new(DDIV {}),
        112 => Box::new(IREM {}),
        113 => Box::new(LREM {}),
        114 => Box::new(FREM {}),
        115 => Box::new(DREM {}),
        116 => Box::new(INEG {}),
        117 => Box::new(LNEG {}),
        118 => Box::new(FNEG {}),
        119 => Box::new(DNEG {}),
        120 => Box::new(ISHL {}),
        121 => Box::new(LSHL {}),
        122 => Box::new(ISHR {}),
        123 => Box::new(LSHR {}),
        124 => Box::new(IUSHR {}),
        125 => Box::new(LUSHR {}),
        126 => Box::new(IAND {}),
        127 => Box::new(LAND {}),
        128 => Box::new(IOR {}),
        129 => Box::new(LOR {}),
        130 => Box::new(IXOR {}),
        131 => Box::new(LXOR {}),
        132 => Box::new(IINC::new00()),
        133 => Box::new(I2L {}),
        134 => Box::new(I2F {}),
        135 => Box::new(I2D {}),
        136 => Box::new(L2I {}),
        137 => Box::new(L2F {}),
        138 => Box::new(L2D {}),
        139 => Box::new(F2I {}),
        140 => Box::new(F2L {}),
        141 => Box::new(F2D {}),
        142 => Box::new(D2I {}),
        143 => Box::new(D2L {}),
        144 => Box::new(D2F {}),
        145 => Box::new(I2B {}),
        146 => Box::new(I2C {}),
        147 => Box::new(I2S {}),
        148 => Box::new(LCMP {}),
        149 => Box::new(FCMPL {}),
        150 => Box::new(FCMPG {}),
        151 => Box::new(DCMPL {}),
        152 => Box::new(DCMPG {}),
        153 => Box::new(IFEQ::new()),
        154 => Box::new(IFNE::new()),
        155 => Box::new(IFLT::new()),
        156 => Box::new(IFGE::new()),
        157 => Box::new(IFGT::new()),
        158 => Box::new(IFLE::new()),
        159 => Box::new(IF_ICMPEQ::new()),
        160 => Box::new(IF_ICMPNE::new()),
        161 => Box::new(IF_ICMPLT::new()),
        162 => Box::new(IF_ICMPGE::new()),
        163 => Box::new(IF_ICMPGT::new()),
        164 => Box::new(IF_ICMPLE::new()),
        165 => Box::new(IF_ACMPEQ::new()),
        166 => Box::new(IF_ACMPNE::new()),
        167 => Box::new(GOTO::new()),

        170 => Box::new(TABLE_SWITCH::new()),
        171 => Box::new(LOOKUP_SWITCH::new()),
        172 => Box::new(IRETURN {}),
        173 => Box::new(LRETURN {}),
        174 => Box::new(FRETURN {}),
        175 => Box::new(DRETURN {}),
        176 => Box::new(ARETURN {}),
        177 => Box::new(RETURN {}),

        178 => Box::new(GET_STATIC::new()),
        179 => Box::new(PUT_STATIC::new()),
        180 => Box::new(GET_FIELD::new()),
        181 => Box::new(PUT_FIELD::new()),

        182 => Box::new(INVOKE_VIRTUAL::new()),
        183 => Box::new(INVOKE_SPECIAL::new()),
        184 => Box::new(INVOKE_STATIC::new()),

        187 => Box::new(NEW::new()),
        188 => Box::new(NEW_ARRAY::new()),
        189 => Box::new(ANEW_ARRAY::new()),
        190 => Box::new(ARRAY_LENGTH::new()),
        192 => Box::new(CHECK_CAST::new()),
        193 => Box::new(INSTANCE_OF::new()),

        196 => Box::new(WIDE::new()),
        197 => Box::new(MULTI_ANEW_ARRAY::new()),
        198 => Box::new(IFNULL::new()),
        199 => Box::new(IFNONNULL::new()),
        200 => Box::new(GOTO_W::new()),
        _ => {
            panic!("Unsupported opcode: {}!", op_code)
        }
    }
}