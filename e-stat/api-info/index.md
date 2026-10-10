---
source: https://www.e-stat.go.jp/api/en/api-info
depth: 2
---

[ Skip to main content ](https://www.e-stat.go.jp/api/en/api-info#main-content)
# [![e-stat API](https://www.e-stat.go.jp/api/themes/custom/estatapi/logo_API_en.png)](https://www.e-stat.go.jp/api/en)
  * [English](https://www.e-stat.go.jp/api/en/api-info?en%2Fapi-info=)
  * [Japanese](https://www.e-stat.go.jp/api/api-info?en%2Fapi-info=)


## 検索フォーム
Search
Search
Enter the terms you wish to search for. 
  * Search
Search
Enter the terms you wish to search for. 
  * [Home](https://www.e-stat.go.jp/api/api/api/api/index.php/en)
  * [Overview »](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-info)
    * [Overview](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-info)
    * [User Guide](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-info/api-guide)
    * [API Specifications](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-info/api-spec)
    * [Data](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-info/api-data)
    * [Theme of Statistics](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-info/statsfield)
    * [Credit](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-info/credit)
  * [Tips for developers »](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-dev)
    * [Tips for developers](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-dev)
    * [Development Guide](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-dev/dev_guide)
    * [How to use API](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-dev/how_to_use)
    * [API Test Form](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-dev/testform)
    * [Samples](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-dev/app-sample)
    * [GitHub](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-dev/github)
    * [Applications developed with our API](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-dev/case)
    * [FAQ](https://www.e-stat.go.jp/api/api/api/api/index.php/en/api-dev/faq)


  1. [Top Page](https://www.e-stat.go.jp/api/en)
  2. Overview 


#  Overview
The API is the REST API which exports statistical data of e-Stat database in machine-readable format (XML, JSON etc).
By developing an application with our API, you will be able to build a sophisticated contents with statistical data.
## List of provided functions
API supports 7 functions below.
### 1. Obtain Information of Statistical Table (GET)
This function provides information of a statistical table. Information include such as table IDs, names of statistics, table names, dates of statistics.  
API narrows information by keywords you set.
### 2. Obtain Meta Information (GET)
This function provides meta information included in statistical tables. (e.g. Tabulated variables, Areas, Classification)
### 3. Obtain Statistical Data (GET)
This function provides statistical data (numerical data) that is in a statistical table.  
Obtained data can be refined by dataset and meta information as required.  
If providing data is numerous, it will be separated.
### 4. Register Dataset (POST)
This function enables users to register refinement conditions when acquiring statistical data.  
It is possible to further narrow down the acquisition conditions of the data set when acquiring statistical data.  
Registered data sets can also be disclosed to other users.
### 5. Dataset Reference (GET)
This function is to refer registered dataset information.
### 6. Obtaining data catalog information (GET)
This function provides a file of statistical table and statistical database information such as survey name, title, links etc.  
It enables to narrow down data catalog by keywords of search likewise the function of statistical table acquisition.
### 7. Batch Acquisition of Statistical Data (POST)
This function provides statistical data (numerical data) in multiple statistical tables at once.  
Obtained data can be refined by dataset and meta information likewise the function of statistical data acquisition.
## Overview
  * [User Guide](https://www.e-stat.go.jp/api/en/api-info/api-guide)
  * [API Specifications](https://www.e-stat.go.jp/api/en/api-info/api-spec)
  * [Data](https://www.e-stat.go.jp/api/en/api-info/api-data)
  * [Theme of Statistics](https://www.e-stat.go.jp/api/en/api-info/statsfield)
  * [Credit](https://www.e-stat.go.jp/api/en/api-info/credit)
  * [Tips for developers](https://www.e-stat.go.jp/api/en/api-dev)


## Links
[ ![e-Stat](https://www.e-stat.go.jp/api/sites/default/files/2022-10/banner_estat-en_0.png) ](https://www.e-stat.go.jp/en/)[ ![Statistics Bureau](https://www.e-stat.go.jp/api/sites/default/files/2022-10/banner_toukei_en_0.gif) ](https://www.stat.go.jp/english/%20)[ ![National Statistics Center](https://www.e-stat.go.jp/api/sites/default/files/2022-10/nstac_logo_en.gif) ](https://www.nstac.go.jp/en/%20)[ ![LOD Challenge 2020](https://www.e-stat.go.jp/api/sites/default/files/2020-08/bnr_LODChallenge2020.png) ](https://2020.lodc.jp/%20)
## About this website
  * [Contact](https://www.e-stat.go.jp/api/en/contact)
  * [Terms of Use](https://www.e-stat.go.jp/api/en/terms-of-use)
  * [Privacy Policy](https://www.e-stat.go.jp/en/privacy)



