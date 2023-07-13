package lance;

import java.nio.file.Path;
import java.nio.file.Paths;
import java.util.Optional;

public class Lance {

    private static native String hello(String input);
    public static native long newDataset(String uri);
    public static native long countRows(long datasetPtr);

    static {
        // This actually loads the shared object that we'll be creating.
        // The actual location of the .so or .dll may differ based on your
        // platform.
        System.loadLibrary("lance");
    }

    public static void main(String[] args) {

        String output = Lance.hello("weijie");
        System.out.println(output);

        LanceDataset lds = LanceDataset.open("/home/weijie-li/data/9940C");
        System.out.println(lds.getDatasetPtr());
        System.out.println(lds.getUri());
        
        long count = lds.countRows();
        System.out.println(count);
    }
}
