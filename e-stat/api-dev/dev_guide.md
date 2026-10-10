---
source: https://www.e-stat.go.jp/api/en/api-dev/dev_guide
depth: 2
---

[ Skip to main content ](https://www.e-stat.go.jp/api/en/api-dev/dev_guide#main-content)
# [![e-stat API](https://www.e-stat.go.jp/api/themes/custom/estatapi/logo_API_en.png)](https://www.e-stat.go.jp/api/en)
  * [English](https://www.e-stat.go.jp/api/en/api-dev/dev_guide?en%2Fapi-dev%2Fdev_guide=)
  * [Japanese](https://www.e-stat.go.jp/api/api-dev/dev_guide?en%2Fapi-dev%2Fdev_guide=)


## 検索フォーム
Search
Search
Enter the terms you wish to search for. 
  * Search
Search
Enter the terms you wish to search for. 
  * [Home](https://www.e-stat.go.jp/api/en)
  * [Overview »](https://www.e-stat.go.jp/api/en/api-info)
    * [Overview](https://www.e-stat.go.jp/api/en/api-info)
    * [User Guide](https://www.e-stat.go.jp/api/en/api-info/api-guide)
    * [API Specifications](https://www.e-stat.go.jp/api/en/api-info/api-spec)
    * [Data](https://www.e-stat.go.jp/api/en/api-info/api-data)
    * [Theme of Statistics](https://www.e-stat.go.jp/api/en/api-info/statsfield)
    * [Credit](https://www.e-stat.go.jp/api/en/api-info/credit)
  * [Tips for developers »](https://www.e-stat.go.jp/api/en/api-dev)
    * [Tips for developers](https://www.e-stat.go.jp/api/en/api-dev)
    * [Development Guide](https://www.e-stat.go.jp/api/en/api-dev/dev_guide)
    * [How to use API](https://www.e-stat.go.jp/api/en/api-dev/how_to_use)
    * [API Test Form](https://www.e-stat.go.jp/api/en/api-dev/testform)
    * [Samples](https://www.e-stat.go.jp/api/en/api-dev/app-sample)
    * [GitHub](https://www.e-stat.go.jp/api/en/api-dev/github)
    * [Applications developed with our API](https://www.e-stat.go.jp/api/en/api-dev/case)
    * [FAQ](https://www.e-stat.go.jp/api/en/api-dev/faq)


  1. [Top Page](https://www.e-stat.go.jp/api/en)
  2. [Tips for developers](https://www.e-stat.go.jp/api/en/api-dev)
  3. Development Guide 


#  Development Guide
We introduce the information required for development with our API on this page.
### 1. Overview
This API provides statistical data, which can be seen on e-Stat, in a machine-readable format.
### 2. The way to use of API
**1. It automatically reflects the data from e-Stat on a users' system.**  
・It always shows the latest information. It also shows main information altogether.  
・It is assumed that the use of reflecting the latest data regularly through the API even if users' system does not keep statistical data.
**2. Retention of users and sophisticated statistical data analysis with being connected with data on the Internet.**  
・It is expected that the use of analysis with BI tools after downloading and storing a bunch of statistical data on a database.  
・Although it takes long time to download a bunch of Excel files and store them in database, it is possible that the use of the API makes storing data in a database easily.  
・The latest information will be dealt with by incorporating differences.
### 3. API Specifications
The API has different URL and parameters respectively by a function.  
We introduce the process of obtaining statistical data by using API.
**[The process of obtaining statistical data]**  
(1) Obtaining a table ID of the target statistical table by using the function of "Obtain Information of Statistical Table API". You are able to look up with date of survey, name, and keywords.
(2) You can use "Obtain Meta Information API" with using table ID acquired from response of (1). You can get meta information of the target statistical table with it.
(3) It gets the target statistical data with using the table ID and meta information as the parameter.
### 4. Data Structure
  * **Obtain Information of Statistical Table**


It obtains information such as title, survey year and table ID for the number of statistical tables.
  * **Obtain Meta Information**


It obtains Classification, Time, Areas and Tabulated variables for the number of meta information.
  * **Obtain Statistical Data**


It exports numerical information of statistical data for the number of cells.
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



