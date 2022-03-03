(*
 * This is a comment.
 * (*
 *  * This is a nested comment.
 *  *)
 *)
CLASS Foo {
    -- This is another line comment
    int_value : Int <- 0;  -- This is another line comment
    bool_value : Bool <- tRUE;
    false_value : Bool <- faLSe;
    str_value : String <- "[\t] [\b] [\\] [\|] [\n]";
    -- This is a line comment
    init(int_val: Int, bool_val: Bool) : SELF_TYPE {
        {
            int_value <- 0 + (int_val / 1) * ~ 1 - 0;
            bool_value <- bool_val;
            false_value <- ISVOID self;
            self;
        }
    };
    int_value() : Int {
        int_value
    };
    bool_value() : Bool {
        bool_value
    };
    false_value() : Bool {
        false_value
    };
    str_value() : String {
        str_value
    };
};

Class Bar INHERITS Foo {
    false_value() : Bool {
        false
    };
};

cLASS Main inHERits IO {
    foo : Foo <- (NEW Foo).init(42, NOT 1 <= 0);
    main() : Int {
	{
        CASE foo OF
            f : Foo => out_string("foo is a Foo\n");
            o : Object => out_string("foo is an Object\n");
        ESAC;

        LET continue : Bool <- foo.bool_value() IN {
            WHILE continue LOOP {
                out_string("Looping...\n");
                continue <- (nEW Bar)@Foo.false_value();
            }
            POOL;
        };

        IF foo.int_value() = 0 THEN {
            out_string("int_value is zero\n");
        } ELSE iF foo.int_value() < 0 tHEN {
            out_string("int_value is negative\n");
        } eLSE {
            out_string("int_value is positive\n");
        } FI fI;

	    out_string("str_value: ".concat(foo.str_value()));
        0;
	}
    };
};
