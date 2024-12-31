# API Documentation

This is documentation related to the request and response formats as well as the routes of this project.

## Table of Contents

- Base API Response
  - [On Success](#on-success)
  - [On Error](#on-error)
- Filter by Parameter
  - [Example](#example)
- Pagination
  - [Pagination](#pagination)
- Endpoint
  - [Clubs](#clubs)
  - [Players](#players)
  - [Matches](#matches)

## Base API Response
- #### **On Success**
  ```json
  {
    "success": true,          // boolean
    "message": "Success ...", // string
    "data": []                // object / array
  }
  ```

- #### **On Error**
  ```json
  {
    "status": false,                        // boolean
    "message": "Invalid request body",      // string
  }
  ```

## Filter by Parameter
There is no filter for sorting yet, but some of endpoint have param filter like name, club_id, etc.
### **Example**
- /club?name=arsenal

Note: this filter is optional. If fetching without this filter, it will show all of the data.

## Pagination
There is no pagination system yet.

# Endpoints

## Clubs

### Get All
URI: `/clubs`

Filter param: `?name={string}`

```json
{
  "success": true,                      // boolean
  "message": "Success fetching clubs",  // string
  "data": [                             // array
    {
      "achievements": [                 // array of strings
        "Premier League Champions: 13 times",
        "FA Cup Winners: 14 times",
        "EFL Cup Winners: 2 times",
        "UEFA Cup Winners' Cup: 1 time"
      ],
      "coach": "Mikel Arteta",         // string
      "founded_year": 1886,            // int
      "id": 1,                         // int
      "logo": "https://xx.svg",        // string (URL, svg format)
      "name": "Arsenal"                // string
    },
    ... 9 more
  ]
}
```

### Get By ID
URI: `/clubs/:id`

```json
{
  "success": true,                      // boolean
  "message": "Success fetching clubs",  // string
  "data": {
    "achievements": [                   // array of strings
      "Premier League Champions: 13 times",
      "FA Cup Winners: 14 times",
      "EFL Cup Winners: 2 times",
      "UEFA Cup Winners' Cup: 1 time"
    ],
    "coach": "Mikel Arteta",         // string
    "founded_year": 1886,            // int
    "id": 1,                         // int
    "logo": "https://xx.svg",        // string (URL, svg format)
    "name": "Arsenal"                // string
  }
}
```

### Get Top Club
URI: `/top-clubs`

```json
{
  "success": true,                         // boolean
  "message": "Success fetching top clubs", // string
  "data": [                                // array
    {
      "achievements": [                    // array of strings
        "Premier League Champions: 20 times",
        "FA Cup Winners: 12 times",
        "EFL Cup Winners: 6 times",
        "UEFA Champions League Winners: 3 times"
      ],
      "coach": "Erik ten Hag",         // string
      "founded_year": 1878,            // int
      "id": 8,                         // int
      "logo": "https://xx.svg",        // string (URL, svg format)
      "name": "Manchester United"      // string
    },
    ... 2 more
  ]
}
```

## Players

### Get All
URI: `/players`

Filter param: `?club_id={int}&name={string}`

```json
{
  "success": true,                       // boolean
  "message": "Success fetching players", // string
  "data": [                              // array
    {
      "age": 25,                         // int
      "club_id": 1,                      // int
      "id": 1,                           // int
      "name": "Aaron Ramsdale",          // string
      "position": "Goalkeeper"           // string
    },
    ... 109 more
  ]
}
```

### Get By ID
URI: `/players/:id`
```json
{
  "success": true,                      // boolean
  "message": "Success fetching player", // string
  "data": {                             // object
    "age": 25,                          // int
    "club": {                           // object
      "achievements": [                 // array of strings
        "Premier League Champions: 13 times",
        "FA Cup Winners: 14 times",
        "EFL Cup Winners: 2 times",
        "UEFA Cup Winners' Cup: 1 time"
      ],
      "coach": "Mikel Arteta",         // string
      "founded_year": 1886,            // int
      "id": 1,                         // int
      "logo": "https://xx.svg",        // string (URL, svg format)
      "name": "Arsenal"                // string
    },
    "id": 1,                            // int
    "name": "Aaron Ramsdale",           // string
    "position": "Goalkeeper"            // string
  }
}
```

## Matches

### Get All
URI: `/matches`

Filter param: `?status={finished|upcoming|current}`

```json
{
  "success": true,                       // boolean
  "message": "Success fetching matches", // string
  "data": [                              // array
    {
      "away_club": {                      // object
        "achievements": [                 // array of strings
          "EFL Championship Runners-Up: 1 time"
        ],
        "coach": "Andoni Iraola",        // string
        "founded_year": 1899,            // int
        "id": 2,                         // int
        "logo": "https://xx.svg",        // string (URL, svg format)
        "name": "Bournemouth"            // string
      },
      "away_score": 1,                  // int
      "date": "2024-11-15",             // string (date format: YYYY-MM-DD)
      "home_club": {                      // object
        "achievements": [                 // array of strings
          "Premier League Champions: 13 times",
          "FA Cup Winners: 14 times",
          "EFL Cup Winners: 2 times",
          "UEFA Cup Winners' Cup: 1 time"
        ],
        "coach": "Mikel Arteta",         // string
        "founded_year": 1886,            // int
        "id": 1,                         // int
        "logo": "https://xx.svg",        // string (URL, svg format)
        "name": "Arsenal"                // string
      },
      "home_score": 3,                  // int
      "id": 1,                          // int
      "location": "Emirates Stadium",   // string
      "status": "finished",             // string enum (finished|upcoming|current)
      "time": "18:00:00"                // string (time format: HH:mm:ss)
    },
    ... 14 more
  ]
}
```

### Get By Id
URI: `/matches/:id`

```json
{
  "success": true,                      // boolean
  "message": "Success fetching match",  // string
  "data": {                             // object
    "away_club": {                      // object
      "achievements": [                 // array of strings
        "EFL Championship Runners-Up: 1 time"
      ],
      "coach": "Andoni Iraola",        // string
      "founded_year": 1899,            // int
      "id": 2,                         // int
      "logo": "https://xx.svg",        // string (URL, svg format)
      "name": "Bournemouth"            // string
    },
    "away_score": 1,                    // int
    "date": "2024-11-15",               // string (date format: YYYY-MM-DD)
    "home_club": {                      // object
      "achievements": [                 // array of strings
        "Premier League Champions: 13 times",
        "FA Cup Winners: 14 times",
        "EFL Cup Winners: 2 times",
        "UEFA Cup Winners' Cup: 1 time"
      ],
      "coach": "Mikel Arteta",         // string
      "founded_year": 1886,            // int
      "id": 1,                         // int
      "logo": "https://xx.svg",        // string (URL, svg format)
      "name": "Arsenal"                // string
    },
    "home_score": 3,                    // int
    "id": 1,                            // int
    "location": "Emirates Stadium",     // string
    "match_stats": {                    // object
      "corners": 5,                     // int
      "fouls": 10,                      // int
      "id": 1,                          // int
      "match_id": 1,                    // int
      "passes": 450,                    // int
      "possession": "55%-45%",          // string
      "shots_on_target": 7              // int
    },
    "status": "finished",               // string
    "time": "18:00:00"                  // string (time format: HH:mm:ss)
  }
}

```