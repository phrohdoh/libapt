enum_from_primitive! {
    #[derive(Debug, PartialEq)]
    pub enum Action
    {
        END = 0x00,
        NEXTFRAME = 0x04,
        PREVFRAME = 0x05,
        PLAY = 0x06,
        STOP = 0x07,
        TOGGLEQUALITY = 0x08,
        STOPSOUNDS = 0x09,
        ADD = 0x0A,
        SUBTRACT = 0x0B,
        MULTIPLY = 0x0C,
        DIVIDE = 0x0D,
        EQUALS = 0x0E,
        LESSTHAN = 0x0F,
        AND = 0x10,
        OR = 0x11,
        NOT = 0x12,
        STRINGEQUALS = 0x13,
        STRINGLENGTH = 0x14,
        SUBSTRING = 0x15,
        POP = 0x17,
        TO_INT = 0x18,
        GETVARIABLE = 0x1C,
        SETVARIABLE = 0x1D,
        SETTARGET2 = 0x20,
        STRINGCONCAT = 0x21,
        GETPROPERTY = 0x22,
        SETPROPERTY = 0x23,
        CLONE_SPRITE = 0x24,
        REMOVE_SPRITE = 0x25,
        TRACE = 0x26,
        STARTDRAGMOVIE = 0x27,
        STOPDRAGMOVIE = 0x28,
        STRINGCOMPARE = 0x29,
        THROW = 0x2A,
        CASTOP = 0x2B,
        IMPLEMENTSOP = 0x2C,
        RANDOM = 0x30,
        MBLENGTH = 0x31, //MB is multibyte strings
        ORD = 0x32, //CHAR to ASCII
        CHR = 0x33, //ASCII to CHAR
        GETTIMER = 0x34,
        MBSUBSTRING = 0x35, //MB is multibyte strings
        MBORD = 0x36, //MB is multibyte strings
        MBCHR = 0x37, //MB is multibyte strings
        DELETE = 0x3A,
        DELETE2 = 0x3B,
        DEFINELOCAL = 0x3C,
        CALLFUNCTION = 0x3D,
        RETURN = 0x3E,
        MODULO = 0x3F,
        NEW = 0x40,
        VAR = 0x41,
        INITARRAY = 0x42,
        INITOBJECT = 0x43,
        TYPEOF = 0x44,
        TARGETPATH = 0x45,
        ENUMERATE = 0x46,
        ADD2 = 0x47, //does also handle strings
        LESSTHAN2 = 0x48,
        EQUALS2 = 0x49,
        TONUMBER = 0x4A,
        TOSTRING = 0x4B,
        PUSHDUPLICATE = 0x4C,
        STACKSWAP = 0x4D,
        GETMEMBER = 0x4E,
        SETMEMBER = 0x4F,
        INCREMENT = 0x50,
        DECREMENT = 0x51,
        CALLMETHOD = 0x52,
        NEWMETHOD = 0x53,
        INSTANCEOF = 0x54,
        ENUMERATE2 = 0x55,
        EA_PUSHTHIS = 0x56,
        EA_PUSHGLOBAL = 0x58,
        EA_PUSHZERO = 0x59,
        EA_PUSHONE = 0x5A,
        EA_CALLFUNCNPOP = 0x5B,
        EA_CALLFUNC = 0x5C,
        EA_CALLMETHODPOP = 0x5D,
        EA_CALLMETHOD = 0x5E,
        BITWISEAND = 0x60,
        BITWISEOR = 0x61,
        BITWISEXOR = 0x62,
        SHIFTLEFT = 0x63,
        SHIFTRIGHT = 0x64,
        SHIFTRIGHT2 = 0x65,
        STRICTEQ = 0x66,
        GREATER = 0x67,
        STRINGGREATER = 0x68,
        EXTENDS = 0x69,
        EA_PUSHTHISVAR = 0x70,
        EA_PUSHGLOBALVAR = 0x71,
        EA_ZEROVAR = 0x72,
        EA_PUSHTRUE = 0x73,
        EA_PUSHFALSE = 0x74,
        EA_PUSHNULL = 0x75,
        EA_PUSHUNDEFINED = 0x76,
        TRACE_START = 0x77,
        GOTOFRAME = 0x81,
        GETURL = 0x83,
        SETREGISTER = 0x87,
        CONSTANTPOOL = 0x88,
        WAITFORFRAME = 0x8A,
        SETTARGET = 0x8B,
        GOTOLABEL = 0x8C,
        WAITFORFRAMEEXPR = 0x8D,
        DEFINEFUNCTION2 = 0x8E,
        TRY = 0x8F,
        WITH = 0x94,
        PUSHDATA = 0x96,
        BRANCHALWAYS = 0x99,
        GETURL2 = 0x9A,
        DEFINEFUNCTION = 0x9B,
        BRANCHIFTRUE = 0x9D,
        CALLFRAME = 0x9E,
        GOTOFRAME2 = 0x9F,     
        EA_PUSHSTRING = 0xA1,
        EA_PUSHCONSTANTBYTE = 0xA2,
        EA_PUSHCONSTANTWORD = 0xA3,
        EA_GETSTRINGVAR = 0xA4,
        EA_GETSTRINGMEMBER = 0xA5,
        EA_SETSTRINGVAR = 0xA6,
        EA_SETSTRINGMEMBER = 0xA7,
        EA_PUSHVALUEOFVAR = 0xAE,
        EA_GETNAMEDMEMBER = 0xAF,
        EA_CALLNAMEDFUNCPOP = 0xB0,
        EA_CALLNAMEDFUNC = 0xB1,
        EA_CALLNAMEDMETHODPOP = 0xB2,
        EA_CALLNAMEDMETHOD = 0xB3,
        EA_PUSHFLOAT = 0xB4,
        EA_PUSHBYTE = 0xB5,
        EA_PUSHSHORT = 0xB6,
        EA_PUSHLONG = 0xB7,
        EA_BRANCHIFFALSE = 0xB8,
        EA_PUSHREGISTER = 0xB9  
    }
}