using Microsoft.AspNetCore.Components;

namespace rsurl_frontend.Services
{
    public class httpClientService
    {
        public httpClientService(HttpClient httpClient, NavigationManager navigationManager)
        {
            HttpClient = httpClient;
            NavigationManager = navigationManager;
            HttpClient.BaseAddress = new Uri(NavigationManager.BaseUri);
        }

        public HttpClient HttpClient { get; }
        public NavigationManager NavigationManager { get; }
    }
}
