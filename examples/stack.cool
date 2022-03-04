(*
 *  Programming Assignment 1
 *    Implementation of a simple stack machine.
 *)

(*
 * This class represents a stack element, which can be an Integer or one of
 * two Operator objects (Addition or Swap).
 *)
class Element inherits IO {
   display() : Object {
      out_string("")
   };
};

class Integer inherits Element {
   value: Int;
   init(val: Int) : SELF_TYPE {
      {
         value <- val;
         self;
      }
   };
   value() : Int {
      value
   };
   display() : Object {
      {
         out_int(value);
         out_string("\n");
      }
   };
};

class Operator inherits Element {
   operate(stack: Stack) : Stack {
      case stack of
         empty : Empty => {
            out_string("Not enough operands on the stack\n");
            abort();
            stack;
         };
         top1 : Top => {
            let operand1 : Element <- top1.peek(),
                rest1 : Stack <- top1.pop()
            in
               {
                  case rest1 of
                     empty : Empty => {
                        out_string("Not enough operands on the stack\n");
                        abort();
                        rest1;
                     };
                     top2 : Top => {
                        let operand2 : Element <- top2.peek(),
                            rest2 : Stack <- top2.pop()
                        in operate_on(rest2, operand1, operand2);
                     };
                  esac;
               };
         };
      esac
   };
   operate_on(stack: Stack, operand1: Element, operand2: Element) : Stack {
      stack
   };
};

class Addition inherits Operator {
   operate_on(stack: Stack, operand1: Element, operand2: Element) : Stack {
      case operand1 of
         integer1 : Integer => {
            case operand2 of
            integer2 : Integer => {
               let new_value : Int <-integer1.value() + integer2.value() in
                  stack.push((new Integer).init(new_value));
            };
            other : Element => {
               out_string("Expected integer on stack for addition\n");
               abort();
               stack;
            };
            esac;
         };
         other : Element => {
            out_string("Expected integer on stack for addition\n");
            abort();
            stack;
         };
      esac
   };
   display() : Object {
      out_string("+\n")
   };
};

class Swap inherits Operator {
   operate_on(stack: Stack, operand1: Element, operand2: Element) : Stack {
      stack.push(operand1).push(operand2)
   };
   display() : Object {
      out_string("s\n")
   };
};

(*
 * This class is the actual stack. It can be either an empty stack (Empty) or
 * the top of a stack (Top) with an element and a pointer to the rest of stack.
 * The push operation can be performed on either sub-class. Only non-empty
 * stacks (Top) support peek and pop operations.
 *)
class Stack {
   push(elem: Element) : Stack {
      (new Top).init(elem, self)
   };
};

class Empty inherits Stack {};

class Top inherits Stack {
   element : Element;
   previous : Stack;
   init(elem: Element, prev: Stack) : SELF_TYPE {
      {
         element <- elem;
         previous <- prev;
         self;
      }
   };
   peek() : Element {
      element
   };
   pop() : Stack {
      previous
   };
};

(*
 * This class represents the possible commands that can be performed on a Stack.
 * The commands can push an integer or an operator (Push), evaluate the top of
 * the stack (Evaluate), or display the contents of the stack (Display). Stop
 * tells the program to terminate. InvalidCommand is used to indicate that an
 * invalid command was entered.
 *)
class StackCommand {
   exec(stack: Stack) : Stack {
      stack
   };
   parse(command: String) : StackCommand {
      if command = "+" then
         (new Push).init(new Addition)
      else if command = "s" then
         (new Push).init(new Swap)
      else if command = "e" then
         new Evaluate
      else if command = "d" then
         new Display
      else if command = "x" then
         new Stop
      else {
         let a2i : A2I <- new A2I in
            if a2i.is_integer(command) then
               (new Push).init((new Integer).init(a2i.a2i(command)))
            else
               new InvalidCommand
            fi;
      }
      fi fi fi fi fi
   };
};

class Push inherits StackCommand {
   element : Element;
   init(elem: Element) : SELF_TYPE {
      {
         element <- elem;
         self;
      }
   };
   exec(stack: Stack) : Stack {
      stack.push(element)
   };
};

class Evaluate inherits StackCommand {
   exec(stack: Stack) : Stack {
      case stack of
         empty : Empty => {
            stack;
         };
         top : Top => {
            case top.peek() of
               operator : Operator => {
                  operator.operate(top.pop());
               };
               other : Element => {
                  top;
               };
            esac;
         };
      esac
   };
};

class Display inherits StackCommand {
   exec(stack: Stack) : Stack {
      case stack of
         empty : Empty => {
            stack;
         };
         top : Top => {
            let elem : Element <- top.peek(),
                rest : Stack <- top.pop()
            in
               {
                  elem.display();
                  exec(rest);
                  top;
               };
         };
      esac
   };
};

class Stop inherits StackCommand {};

class InvalidCommand inherits StackCommand {};

(*
 * This is the main program. It first creates and empty stack. Then it reads and
 * parses the user's input into StackCommand objects, and executes until a
 * Stop or InvalidCommand is produced.
 *)
class Main inherits IO {
   stack : Stack <- (new Empty);
   command : StackCommand <- new StackCommand;

   main() : Object {
      let continue : Bool <- true in {
         while continue loop {
            {
               out_string(">");
               let cmd : StackCommand <- command.parse(in_string()) in
                  case cmd of
                     cmd : Stop => {
                        continue <- false;
                     };
                     cmd : InvalidCommand => {
                        out_string("Invalid stack command\n");
                        continue <- false;
                     };
                     cmd : StackCommand => {
                        stack <- cmd.exec(stack);
                     };
                  esac;
            };
         }
         pool;
      }
   };
};
