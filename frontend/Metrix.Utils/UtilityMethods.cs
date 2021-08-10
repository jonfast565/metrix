using Newtonsoft.Json.Serialization;
using System;

namespace Metrix.Utils
{
    public class UtilityMethods
    {
        public static DefaultContractResolver GetDefaultJsonContractResolver()
        {
            return new DefaultContractResolver
            {
                NamingStrategy = new SnakeCaseNamingStrategy()
            };
        }
    }
}
