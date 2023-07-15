public class Main {
    public static void main(String[] args) {
        AbstractAdder adder = new AbstractAdder() {
            @Override
            public int add(int a, int b) {
                return a + b
            }
        };

        int result = adder.add(5,10);
        System.out.println("The result is: " + result);
    }
}

abstract class AbstractAdder implements AdderInterface {
    @Override
    public abstract int add(int a, int b);
}

interface AdderInterface {
    int add(int a, int b)
}
