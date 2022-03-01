(*
 * This is a comment.
 * (*
 *  * This is a nested comment.
 *  *)
 *)
class Foo {
    -- This is another line comment
    int_value : Int <- 0;  -- This is another line comment
    bool_value : Bool <- true;
    false_value : Bool <- false;
    str_value : String <- "[\t] [\b] [\\] [\0] [\|] [\n]";
    -- This is a line comment
    init(int_val: Int, bool_val: Bool) : SELF_TYPE {
        {
            int_value <- 0 + (int_val / 1) * ~ 1 - 0;
            bool_value <- bool_val;
            false_value <- isvoid self;
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

class Bar inherits Foo {
    false_value() : Bool {
        false
    };
};

class Main inherits IO {
    foo : Foo <- (new Foo).init(42, not 1 <= 0);
    main() : Int {
	{
        case foo of
            f : Foo => out_string("foo is a Foo\n");
            o : Object => out_string("foo is an Object\n");
        esac;

        let continue : Bool <- foo.bool_value() in {
            while continue loop {
                out_string("Looping...\n");
                continue <- (new Bar)@Foo.false_value();
            }
            pool;
        };

        if foo.int_value() = 0 then {
            out_string("int_value is zero\n");
        } else if foo.int_value() < 0 then {
            out_string("int_value is negative\n");
        } else {
            out_string("int_value is positive\n");
        } fi fi;

	    out_string("str_value: ".concat(foo.str_value()));
        0;
	}
    };
};
