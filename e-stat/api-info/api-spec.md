---
source: https://www.e-stat.go.jp/api/en/api-info/api-spec
depth: 2
---

[ Skip to main content ](https://www.e-stat.go.jp/api/en/api-info/api-spec#main-content)
# [![e-stat API](https://www.e-stat.go.jp/api/themes/custom/estatapi/logo_API_en.png)](https://www.e-stat.go.jp/api/en)
  * [English](https://www.e-stat.go.jp/api/en/api-info/api-spec?en%2Fapi-info%2Fapi-spec=)
  * [Japanese](https://www.e-stat.go.jp/api/api-info/api-spec?en%2Fapi-info%2Fapi-spec=)


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
  2. [Overview](https://www.e-stat.go.jp/api/en/api-info)
  3. API Specifications 


#  API Specifications
See below for the details of API specifications.
## API Specifications (Released on 2019/7/26) [![new](https://www.e-stat.go.jp/api/sites/default/files/inline-images/new.png)](https://www.e-stat.go.jp/api/sites/default/files/inline-images/new.png)
The specification is to provide statistical data in XML, JSON, and CSV formats.([Word](https://www.e-stat.go.jp/api/sites/default/files/uploads/2019/07/API-specVer3.0.docx "API specifications\(word\)-ver3.0") / [PDF](https://www.e-stat.go.jp/api/sites/default/files/uploads/2019/07/API-specVer3.0.pdf "API specifications\(pdf\)-ver3.0") / [HTML](https://www.e-stat.go.jp/api/api-info/e-stat-manual3-0 "API specifications"))(Japanese Only)
version: 3.0  
Hostname: api.e-stat.go.jp  
Request URL: http://api.e-stat.go.jp/rest/3.0/app…
### About the changes from version 2.1
**1. Added the ability to retrieve data in CSV format.**
・Added the ability to retrieve data in CSV format to the statistical table information retrieval function and meta information retrieval function.
**2. Add aggregate region categories**
・Added a parameter called "collectArea" to specify the aggregate region category, as well as the output of the aggregate region category.
**3. Add description of the statistical table, provided statistics, provided classifications, and metadata.**
・Added a parameter "explanationGetFlg" to specify whether or not to get an explanation for the statistical table, provided statistics, provided classification and metadata, and an output of the explanation.
**4. Add annotations to statistical data**
・Added a parameter "annotationGetFlg" to specify whether or not to get annotation information in the statistical data acquisition function and the statistical data bulk acquisition function, and we have also added the output of annotations.
**5. The provision of the System of Social and Demographic Statistics will be discontinued from the search data type.**
・The search data type "3: System of Social and Demographic Statistics (Regional Statistics Database)" is not required to obtain statistical table information for the System of Social and Demographic Statistics in the search data type parameter "searchKind" of the statistical table information acquisition function. Therefore, the search data type was discontinued.  
Note: 1. Statistical information and 2. Small area and regional mesh are still available.
## API Specifications (Released on 2016/7/14)
The specification is to provide statistical data in XML, JSON, and CSV formats.([Word](https://www.e-stat.go.jp/api/sites/default/files/uploads/2016/07/API-specVer2.1.docx "API specifications\(word\)-ver2.1") / [PDF](https://www.e-stat.go.jp/api/sites/default/files/uploads/2016/07/API-specVer2.1.pdf "API specifications\(pdf\)-ver2.1") / [HTML](https://www.e-stat.go.jp/api/api-info/e-stat-manual2-1 "API secifications"))(Japanese Only)
version: 2.1  
Hostname: api.e-stat.go.jp  
Request URL: http://api.e-stat.go.jp/rest/2.1/app…
### About the changes from version 2.0
**1. Added a function to obtain statistical data in CSV format.**
・Added the ability to download statistical data in CSV format to the statistical data acquisition function.
**2. Addition of statistical data bulk acquisition function**
・Added a function that allows you to acquire multiple statistical data at once.
**3. Added a tag that splits and outputs the provider category name and statistical table name in the statistical table information for each function.**
・Added the STATISTICS_NAME_SPEC and TITLE_SPEC tags.
## API Specifications (Released on 2015/1/30)
The specification is to provide statistical data in XML, JSON, and CSV formats.([Word](https://www.e-stat.go.jp/api/sites/default/files/uploads/2015/04/API-specVer2.0.docx "API specifications\(word\)-ver2.0") / [PDF](https://www.e-stat.go.jp/api/sites/default/files/uploads/2015/04/API-specVer2.0.pdf "API specifications\(pdf\)-ver2.0") / [HTML](https://www.e-stat.go.jp/api/api-info/e-stat-manual "API specifications"))(Japanese Only)
version: 2.0  
Hostname: api.e-stat.go.jp  
Request URL: http://api.e-stat.go.jp/rest/2.0/app…
### About the changes from version 1.0
**1. Added data catalog information retrieval function**
・Added a new feature that allows you to get a list of all statistical data (including Excel) provided on e-Stat through an API. The list includes information such as the survey name, title, and link information.
**2. All API functions are now available in JSON format.**
・The JSON format is now supported for all API functions, which were previously only implemented for the statistical data acquisition function.
**3. Added functions such as providing update information for statistical table information acquisition.**
・Added a parameter "updatedDate" to specify the update date for the statistical table information acquisition function.
・Added the operator "NOT" to the parameter "searchWord.searchKeyword".  
【Parameter Example】  
Note: Please URL-encode all characters you specify.  
・To search statistics that do not include "Public Internals" in the "School Basic Survey"  
searchWord=Basic School Survey NOT Public Schools  
・If you want to search for statistics that do not include "Public School Breakdown" and "National, Public and Private [Total]" in the "Basic School Survey"  
searchWord=Basic School Survey NOT (Public School Composition OR National, Public, and Private Schools [Total])
  * **4. Fix some of the output data, such as unifying the output tags of the statistical table information for each function.**


・Unified the "Statistical Table Information.TABLE_INF tag" in the statistical table information retrieval, metadata retrieval, statistical data retrieval, and dataset reference functions. Please refer to Table 1 for more details.
・Added the MAIN_CATEGORY tag, SUB_CATEGORY tag, OVERALL_TOTAL_NUMBER tag and UPDATED_DATE tag to the TABLE_INF tag of the statistical table information for each function.
・Changed the LIST_INF tag to TABLE_INF tag for obtaining statistical table information.
・Changed the LIST_INF tag to the DATASET_INF tag for the dataset reference.
・Added the RESULT_INF tag to the statistical table information retrieval and statistical data retrieval.  
【Output information for the RESULT_INF tag】  
Statistical table information retrieval: FROM_NUMBER tag, TO_NUMBER tag and NEXT_KEY tag  
Statistical data retrieval: TOTAL_NUMBER tag, FROM_NUMBER tag, TO_NUMBER tag and NEXT_KEY tag
**Table 1：4. About the TABLE_INF tag of statistical table information for each function**  
| Tag  | Version 1.0  | Version  
2.0  |  
| --- | --- | --- |  
| Statistical table information  
retrieval  | Meta information  
retrieval  | Statistical data  
retrieval  | Dataset  
reference  |  
| STAT_NAME  | ●  | ●  | ●  | ●  | ●  |  
| GOV_ORG  | ●  | ●  | ●  | ●  | ●  |  
| STATISTICS_NAME  | ●  | ●  | ●  | ●  | ●  |  
| TITLE  | ●  | ●  | ●  | ●  | ●  |  
| CYCLE  | ●  |   |   |   | ●  |  
| SURVEY_DATE  | ●  | ●  | ●  |   | ●  |  
| OPEN_DATE  | ●  |   |   |   | ●  |  
| SMALL_AREA  | ●  |   |   |   | ●  |  
| MAIN_CATEGORY  |   |   |   |   | ●  |  
| SUB_CATEGORY  |   |   |   |   | ●  |  
| OVERALL_TOTAL_NUMBER  |   |   |   |   | ●  |  
| UPDATED_DATE  |   |   |   |   | ●  |  
## API Specifications (Released on 2014/10/31)
The specification is to provide statistical data in XML and JSON formats([Word](https://www.e-stat.go.jp/api/sites/default/files/uploads/2014/12/API-specVer1.0.docx "API specifications\(word\)-ver1.0") / [PDF](https://www.e-stat.go.jp/api/sites/default/files/uploads/2014/12/API-specVer1.0.pdf "API specifications\(pdf\)-ver1.0"))(Japanese Only)
Version 1.0  
Hostname: api.e-stat.go.jp  
Request URL: http://api.e-stat.go.jp/rest/1.0/app…
## Reduction of communication volume by gzip compression
The response for each request URL in the API is compressed in gzip format.  
Getting API responses in gzip format can reduce the amount of data transferred, which can lead to faster processing. To receive a gzip-compressed API response, you need to set the HTTP request header (the "Accept-Encoding" header) to "gzip" and request the API.  
Note: gzip-compressed API responses need to be decompressed separately.
(Setting for HTTP request header) Accept-Encoding: gzip
## Cross-Origin Resource Sharing (CORS) support
The following request URLs of the API support Cross-Origin Resource Sharing (CORS).  
Therefore, if you are using a browser that supports XMLHttpRequset (Level 2), you can use XMLHttpRequest to communicate with other domains.
Supported Request URL: http://api.e-stat.go.jp/rest/<version>/app/json/…
## https support
As of January 4, all versions of the API function (1.0, 2.0, 2.1, and 3.0) are now available via "https".  
Note: "http" requests are still available.
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



