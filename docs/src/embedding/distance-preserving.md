# Distance Preserving Encryption

DPE" stands for "Distance Preserving Encryption," which refers to a type of encryption scheme designed to maintain the relative distances between data points even after encryption, meaning the distance calculated between two encrypted pieces of data should be the same as the distance between their original, unencrypted counterparts.


YAK implements a form of property preserving encryption to encrypt vector embeddings in such a way that they can still be used for the common use cases of nearest neighbor search and clustering based on their similarity to other vectors. The property that is preserved by the encryption is the distance between vectors. Like other forms of property-preserving encryption, a tradeoff is made between the most secure protection of the data and the usability of the data in its encrypted form.


## Definition
Let D be a data set, d be a distance measure and Enc an encryption algorithm for data items in D. Then, Enc is d-distance preserving if ∀x,y ∈ D : d(Enc(x), Enc(y)) = d(x,y). Distance-preserving encryption enables distance-based data mining on encrypted data sets.


## DPE in Theory

### Encrypting Vector Elements


The core idea of the technique is to scale the elements of the vector by a secret factor, then to perturb the elements of the scaled vector by adding a random vector to it. This perturbation is pseudorandom, based on a secret key. The idea is that the amount of perturbation of each individual element is uniformly distributed within a range defined by a value that we refer to as the approximation factor. Conceptually, for a vector with n elements, the encryption can be viewed as choosing a new point that lies within an n-dimensional sphere centered on the point represented by the vector whose radius is determined by the scaling factor and approximation factor. The choice of the point is pseudorandom rather than fully random so that the encrypted vector can be decrypted given the same key.


The larger the approximation factor, the more difficult it is to guess what the original point was, but the less accurate the results of nearest neighbor searches are. The choice of approximation factor must balance the need for security with the need for accuracy.

### Shuffling Vector Elements

After the vector has been encrypted, we reorder the elements of the vector by applying a deterministic shuffling algorithm that is based on a secret value. Like the choice of scaling factor and secret used to encrypt the vector elemetns, the same shuffling secret is used for all the vectors in the same data segment, but a different secret is used for each segment.

