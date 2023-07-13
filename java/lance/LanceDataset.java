package lance;

import java.util.Optional;
import java.lang.Integer;

public interface LanceDataset {
    String getUri();

    long getDatasetPtr();

    long countRows();

    // default LanceDataset create(String uri, Optional<Integer> version, Optional<Integer> blockSize, Optional<Integer> indexCacheSize) {
    //     long dataset = Lance.newDataset(uri, version, blockSize, indexCacheSize);
    //     return new LanceDatasetImpl(uri, dataset);
    // }

    static LanceDataset open(String uri) {
        return new LanceDatasetImpl(uri, Lance.newDataset(uri));
    }

    class LanceDatasetImpl implements LanceDataset {
        private final String uri;
        private final long dataset;
    
        public LanceDatasetImpl(String uri, long dataset) {
            this.uri = uri;
            this.dataset = dataset;
        }
    
        @Override
        public String getUri() {
            return uri;
        }
    
        @Override
        public long getDatasetPtr() {
            return dataset;
        }
    
        public long countRows() {
            return Lance.countRows(getDatasetPtr());
        }
    }
    
}

