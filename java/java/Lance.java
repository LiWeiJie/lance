public class Lance {

    private static native String hello(String input);

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("lance");
    }

    public static void main(String[] args) {

        String output = Lance.hello("josh");
        System.out.println(output);
    

        // HelloWorld.factAndCallMeBack(6, new HelloWorld());

        // long counter_ptr = counterNew(new HelloWorld());

        // for (int i = 0; i < 5; i++) {
        //   counterIncrement(counter_ptr);
        // }

        // counterDestroy(counter_ptr);

        // System.out.println("Invoking asyncComputation (thread id = " + Thread.currentThread().getId() + ")");
        // asyncComputation(new HelloWorld());
    }

}
