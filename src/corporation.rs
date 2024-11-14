use crate::model::corporation::Corporation;
use crate::Client;

impl Client {
    pub async fn get_corporation(
        &self,
        corporation_id: i32,
    ) -> Result<Corporation, reqwest::Error> {
        let url = format!(
            "{}/corporations/{}/?datasource=tranquility",
            self.esi_url, corporation_id
        );

        self.get_from_public_esi::<Corporation>(&url).await
    }
}

#[cfg(test)]
mod tests {
    static USER_AGENT: &str = concat!(env!("CARGO_PKG_NAME"), "/", env!("CARGO_PKG_VERSION"));

    #[tokio::test]
    async fn get_corporation() {
        let mut mock_server = mockito::Server::new_async().await;

        let mock_server_url = mock_server.url();

        let expected_corporation = crate::model::corporation::Corporation {
            alliance_id: Some(99013534),
            ceo_id: 2114794365,
            creator_id: 2114794365,
            date_founded: Some("2024-10-07T21:43:09Z".parse().unwrap()),
            description: Some("".to_string()),
            home_station_id: Some(60003760),
            member_count: 21,
            name: "The Order of Autumn".to_string(),
            shares: Some(1000),
            tax_rate: 0.0,
            ticker: "F4LL.".to_string(),
            url: Some("https://autumn-order.com".to_string()),
            war_eligible: Some(true),
            faction_id: None,
        };

        let mock = mock_server.mock("GET", "/corporations/98785281/?datasource=tranquility")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(r#"{"alliance_id": 99013534, "ceo_id": 2114794365, "creator_id": 2114794365, "date_founded": "2024-10-07T21:43:09Z", "description": "", "home_station_id": 60003760, "member_count": 21, "name": "The Order of Autumn", "shares": 1000, "tax_rate": 0, "ticker": "F4LL.", "url": "https://autumn-order.com", "war_eligible": true}"#)
            .create();

        let mut esi_client: crate::Client = crate::Client::new(USER_AGENT);

        esi_client.esi_url = mock_server_url.to_string();

        let corporation = esi_client.get_corporation(98785281).await.unwrap();

        mock.assert();

        assert_eq!(corporation, expected_corporation);
    }

    #[tokio::test]
    async fn get_corporation_not_found() {
        let mut mock_server = mockito::Server::new_async().await;

        let mock_server_url = mock_server.url();

        let mock = mock_server
            .mock("GET", "/corporations/99999999/?datasource=tranquility")
            .with_status(404)
            .with_header("content-type", "application/json")
            .with_body(r#"{"error": "Corporation not found"}"#)
            .create();

        let mut esi_client: crate::Client = crate::Client::new(USER_AGENT);

        esi_client.esi_url = mock_server_url.to_string();

        let result = esi_client.get_corporation(99999999).await;

        mock.assert();

        assert!(result.is_err());
    }
}
