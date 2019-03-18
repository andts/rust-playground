#BigFile

Lists all files/dirs under current dir, sorted by their significant size.

Significant size is:
* a size of the file
* a size of all files in a dir (excluding big files), disregarding the sub-dirs  
* a big file is a 
  * file of size that is more than 90% of the dir size (excluding sub-dirs)
  * an outlier in the distribution of file sizes?
  * 200% of a median file size?
  * ... ? 

A tie in significant sizes of entries is resolved by alphabetical order of entry path.
  
Output example:
---------------
    dir1/bigfile1           99  (99) 
    dir1                    51 (170)
    dir1/subdir1            12  (12)
    dir1/subdir2             8   (8)
    dir1/smallfile1          1
    ...
    dir1/smallfile51         1