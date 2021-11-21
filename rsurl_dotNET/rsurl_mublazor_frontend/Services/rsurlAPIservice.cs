
using System.Net.Http.Headers;

namespace rsurl_mublazor_frontend.Services
{
    public class rsurlAPIservice
    {
        private readonly HttpClient client;

        public rsurlAPIservice(HttpClient client)
        {
            this.client = client;
        }

        public async Task<(string, string)> GetShortURL(string url)
        {
            List<KeyValuePair<string, string>> keyValuePairs = new List<KeyValuePair<string, string>>();
            keyValuePairs.Add(new KeyValuePair<string, string>("url", url));
            HttpContent content = new FormUrlEncodedContent(keyValuePairs);
            content.Headers.ContentType = new MediaTypeHeaderValue("application/x-www-form-urlencoded");
            content.Headers.ContentType.CharSet = "UTF-8";
            //client.DefaultRequestHeaders.ExpectContinue = false;
            HttpResponseMessage response = await client.PostAsync(new Uri("http://localhost:8000/shorten/"), content);

            //if (response.StatusCode == System.Net.HttpStatusCode.OK)
            //{
                var responseContent = await response.Content.ReadAsStringAsync();
                return (response.StatusCode.ToString(), responseContent);
            //}
            //else
            //{
            //    return (response.StatusCode.ToString(), response.Content);
            //}
        }
    }
}
