---
source: https://www.e-stat.go.jp/api/en/api-dev/how_to_use
depth: 2
---

[ Skip to main content ](https://www.e-stat.go.jp/api/en/api-dev/how_to_use#main-content)
# [![e-stat API](https://www.e-stat.go.jp/api/themes/custom/estatapi/logo_API_en.png)](https://www.e-stat.go.jp/api/en)
  * [English](https://www.e-stat.go.jp/api/en/api-dev/how_to_use?en%2Fapi-dev%2Fhow_to_use=)
  * [Japanese](https://www.e-stat.go.jp/api/api-dev/how_to_use?en%2Fapi-dev%2Fhow_to_use=)


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
  3. How to use API 


#  How to use API
We introduce how to use API with some sample codes.
The sample below is to display graphs based on the data obtained from Obtain Statistical Data API.
## Run a API request
By running the request below, you can get the data of the percentage of people aged over 65.

```

http://api.e-stat.go.jp/rest/2.0/app/getStatsData?appId=<application ID>&statsDataId=C0020050213000&cdCat01=%23A03503

```

### About URL
In case of Obtain Statistical Data API, the URLs are as follows. 
```

// XML
http://api.e-stat.go.jp/rest/<version>/app/getStatsData?<parameters>

// JSON
http://api.e-stat.go.jp/rest/<version>/app/json/getStatsData?<parameters>





```

The example request above, which is the percentage of people aged over 65, gets XML.
Please refer the page below in regard to the data types of Obtain Statistical Data API.
[e-Stat API specifications/2. How to use API/2.3. Obtain Statistical Data API](https://www.e-stat.go.jp/api/api-info/e-stat-manual/#api_2_3 "e-Stat API specifications/2. How to use API/2.3. Obtain Statistical Data API")
(in Japanese)
### About parameters
When running a request, setting parameter is required. Each parameter should be combined a name and value with "=" such as
"name=value".
In case of setting multiple values, conbine them with "&" such as
"name1=value1&name2=value2&…".
Parameters must be URL encoded (UTF-8).
For instance, the code of the percentage of people aged over 65 in "System of Social and Demographic Statistics" is
#A03503.
It will be
%23A03503
after URL encoded.
### Parameters setting
Application ID is required when you use API released on e-Stat.
Assign an application ID in the request.

```
appId=<application ID>
```

To issue an application ID, you need to sign up on e-Stat.
Please sign up from the link below.
[Sign up](https://www.e-stat.go.jp/en/mypage/user/preregister/ "Sign up")
Assign table ID of data you want.
Set C0020050201300 in case of System of Social and Demographic Statistics of Tokyo.

```
statsDataId=C0020050213000
```

There are a bunch of items in System of Social and Demographic Statistics.
Set the ID of the item you like to get. If you do not assign it, API returns all items.
In the example below, it shows the set of parameter to get the data of the percentage of people aged over 65.

```
cdCat01=%23A03503
```

#### About omittable parameters
You can omit parameters except for application ID and statsDataId.
There are the parameters that have default value such as lang and statsNameList.
In regard to Obtain Statistical Data API, refer to the page below.
[e-Stat API specifications/3. API parameter/3.4. Obtain Statistical Data](https://www.e-stat.go.jp/api/api-info/e-stat-manual/#api_3_4 "e-Stat API specifications/3. API parameter/3.4. Obtain Statistical Data")
(in Japanese)
Below code is the PHP example for using Obtain Statistics Data API.

```

// Application ID
$APP_ID = <application ID>;
// API URL
$API_URL = "http://api.e-stat.go.jp/rest/2.0/app/getStatsData";
// Table ID
$statsDataId ="C0020050213000"
// Code
$cdCat01 ="#A03503"

// Set parameters of URL
$GET_URL = $API_URL;
$GET_URL .= "?appId=". urlencode($APP_ID);
$GET_URL .= "&statsDataId=". urlencode($statsDataId);
$GET_URL .= "&cdCat01=". urlencode($cdCat01);



```

Below code is the Javascript example for using Obtain Statistics Data API.

```

// Application ID
var APP_ID = <application ID>;
// API URL
var API_URL = "http://api.e-stat.go.jp/rest/2.0t/app/json/getStatsData"
// Table ID
var statsDataId ="C0020050213000"
// Code
var cdCat01 ="#A03503"

var GET_URL = API_URL;
GET_URL += "?appId=" + escape(APP_ID);
GET_URL += "&statsDataId=" + escape(statsdataid);
GET_URL += "&cdCat01=" + escape(cdcat01);



```

## Analysis of exported data
Data are exported text format such as XML or JSON.
The sample below is to get XML data by "simplexml_load_file" of PHP,
and to anylyse required data by XPath.
Refer to the page below in regard to the contents of response from Obtain Statistical Data API.
[e-Stat API specifications/4. Data exported from API/4.4. Obtain Statistical Data API](https://www.e-stat.go.jp/api/api-info/e-stat-manual/#api_4_4 "e-Stat API specifications/4. Data exported from API/4.4. Obtain Statistical Data API")
(in Japanese)

```

// Obtain XML data
$xmlData =  simplexml_load_file( $GET_URL );

// Obtain result status from array
$apiResult = get_object_vars($xmlData->RESULT);
$apiStatus = $apiResult["STATUS"];
$apiErrMsg = $apiResult["ERROR_MSG"];

// Define variable
$aryGraph = array();

// Area（Prefecture code + "000"）
$areaCode = "13000";  // 13000 is the code of Tokyo

// Data acquisition succeeded
if ($apiStatus == "0") {
  // Obtain table name from meta info
  $statisticsNameList=$xmlData->xpath('/GET_STATS_DATA/STATISTICAL_DATA/TABLE_INF/STATISTICS_NAME');
  $statisticsName=$statisticsNameList[0];

  // Obtain prefecture name from meta info
  $areaNameList=$xmlData->xpath('/GET_STATS_DATA/STATISTICAL_DATA/CLASS_INF/CLASS_OBJ[@id="area"]/CLASS[@code="'.$areaCode.'"]/@name');
  $areaName=(string)$areaNameList[0];

  // Obtain classification from meta info
  $cat01List=$xmlData->xpath('/GET_STATS_DATA/STATISTICAL_DATA/CLASS_INF/CLASS_OBJ[@id="cat01"]/CLASS[@code="'.$cdCat01.'"]/@name');
  $cat01=$cat01List[0];

  // Obtain unit
  $UnitList=$xmlData->xpath('/GET_STATS_DATA/STATISTICAL_DATA/CLASS_INF/CLASS_OBJ[@id="cat01"]/CLASS[@code="'.$cdCat01.'"]/@unit');
  $Unit=$UnitList[0];

  // The array for storing horizontal axis of meta info
  $apiMetaYokoList = array();

  // Obtain horizontal axis from meta info
  foreach($xmlData->xpath('/GET_STATS_DATA/STATISTICAL_DATA/CLASS_INF/CLASS_OBJ[@id="time"]/CLASS') as $key1 => $val1) {
    // store
    $apiMetaYokoList[(string)$val1['code']] = (string)$val1['name'];
  }

  // Modify obtained data for displaying
  foreach ($xmlData->xpath('/GET_STATS_DATA/STATISTICAL_DATA/DATA_INF/VALUE') as $key2 => $val2) {
  // Loop with data

    // Loop with obtained horizontal axis
    foreach ($apiMetaYokoList as $yokoKey => $yokoVal) {

      if ((($cdCat01!=""&&(string)$val2['cat01']==$cdCat01)||$cdCat01=="") &&    // Classification 01
        (string)$val2['area'] == $areaCode &&                                    // Prefectures
        (string)$val2['time'] == $yokoKey                                        // horizontal axis
      ) {
        //
        $name = $apiMetaYokoList[$yokoKey];

        // Convert obtained value to number
        $value = (float)$val2;

        // Store data for graph in the array
        $aryGraph[] = array(  "name"  => $name
                            , "value" => $value
        );
      }
    }
  }
}


```

The sample below is to analyse the data obtained by getJSON of Javascript(JQuery).

```

$.getJSON(url, function(jsonData){})
.success(function(jsonData) {
  var statusCode = jsonData['GET_STATS_DATA']['RESULT']['STATUS'];

  // Extract numerical information of statistical data
  // Extract data from json data by filter of Javascript
  var valueData = jsonData['GET_STATS_DATA']['STATISTICAL_DATA']['DATA_INF']['VALUE'].filter(function(item, index){
    return true;
  });

  // Obtain horizontal axis from meta info
  var apiMetaYokoList = jsonData['GET_STATS_DATA']['STATISTICAL_DATA']['CLASS_INF']['CLASS_OBJ'].filter(function(item, index){
    if (item['@id'] == 'time'){
      return true;
    }
  });

  // Obtain unit
  var apiMetaCat01List = jsonData['GET_STATS_DATA']['STATISTICAL_DATA']['CLASS_INF']['CLASS_OBJ'].filter(function(item, index){
    if (item['@id'] == 'cat01'){
      return true;
    }
  });
  var Unit = apiMetaCat01List[0]['CLASS']['@unit'];

  // The array to store the data to draw on a graph
  var graphData = new Array();
  for (var key in valueData) {
    // Meta info
    var apiYokoName = apiMetaYokoList[0]['CLASS'].filter(function(item, index){
      if (item['@code'] == valueData[key]['@time']){
        return true;
      }
    });
    graphData.push({
      name : apiYokoName[0]['@name'],
      value: valueData[key]['$']
    });
  }
});



```

## Export to graphs
Based on the analysed data, API draws graphs by using Javascript library.
Morris is used in the sample below.
It is useful to choose JSON format when passing data from PHP to Javascript.
The graph is drawn in the DIV tag which ID is "mygraph" in this sample.

```

<script>
  // Draw graph
  new Morris.Bar({
      element: 'mygraph'
      , data: JSON.parse('<?php echo json_encode($aryGraph);?>')
      , xkey: 'name'
      , ykeys: ['value']
      , xLabels: "Year"
      , labels: ['value']
      , postUnits: '<?php echo $Unit;?>'
      , gridLineColor: '#000000'
      , hideHover: 'auto'
  });
</script>
<div id="mygraph"></div>



```

Pass the data as is for Javascript.

```

<script>
  // Draw graph
  new Morris.Bar({
      element: 'mygraph'
    , data: graphData
    , xkey: 'name'
    , ykeys: ['value']
    , xLabels: "Year"
    , labels: ['value']
    , postUnits: Unit
    , gridLineColor: '#000000'
    , hideHover: 'auto'
  });
</script>
<div id="mygraph"></div>



```

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



