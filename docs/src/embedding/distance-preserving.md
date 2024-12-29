# Distance Preserving Encryption

DPE" stands for "Distance Preserving Encryption," which refers to a type of encryption scheme designed to maintain the relative distances between data points even after encryption, meaning the distance calculated between two encrypted pieces of data should be the same as the distance between their original, unencrypted counterparts.


## Definition
Let D be a data set, d be a distance measure and Enc an encryption algorithm for data items in D. Then, Enc is d-distance preserving if ∀x,y ∈ D : d(Enc(x), Enc(y)) = d(x,y). Distance-preserving encryption enables distance-based data mining on encrypted data sets.


# Use DPE on RAG 
