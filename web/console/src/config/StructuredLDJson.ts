/*!
=========================================================
* © 2024 Ronan LE MEILLAT for SCTG Development
=========================================================
This website use:
- Vuejs v3
- Font Awesome
- And many others
*/
import { $require } from "@/utilities/viteHelper"

const daysOfWeek = [
  "Monday",
  "Tuesday",
  "Wednesday",
  "Thursday",
  "Friday",
  "Saturday",
  "Sunday",
];

function getStartOfSeason(): string {
  const now = new Date();
  const curMonth = now.getMonth();
  if (curMonth > 4) return `${now.getFullYear()}-12-01`;
  else return `${now.getFullYear() - 1}-12-01`;
}
function getEndOfSeason(): string {
  const now = new Date();
  const curMonth = now.getMonth();
  if (curMonth > 4) return `${now.getFullYear() + 1}-04-30`;
  else return `${now.getFullYear()}-04-30`;
}
declare type OpeningHoursSpecification = {
  "@type": string;
  validFrom: string;
  validThrough: string;
  dayOfWeek?: string;
  opens: string;
  closes: string;
};
function getOpeningHours(): OpeningHoursSpecification[] {
  const oh = [];
  daysOfWeek.forEach((day) => {
    oh.push({
      "@type": "OpeningHoursSpecification",
      validFrom: getStartOfSeason(),
      validThrough: getEndOfSeason(),
      dayOfWeek: `https://schema.org/${day}`,
      opens: "09:00:00",
      closes: "17:00:00",
    });
  });
  return oh;
}

export const addJsonLD = () => {
  const jsonld = {
    "@context": "https://schema.org",
    "@type": "SportsClub",
    description: "Wonderful template",
    name: "Highcanfly wonderful template",
    alternateName: ["High Can Fly", "Dev team"],
    naics: "711211",
    image: $require('@/assets/vue.svg'),
    logo: $require('@/assets/vue.svg'),
    telephone: "+33 1 23 45 67 89",
    nonprofitStatus: "Nonprofit501c7",
    url: new URL(document.location.href).origin,
    address: {
      "@type": "PostalAddress",
      streetAddress: "allée du bon repos",
      postalCode: "99999",
      addressLocality: "BONNE-SIESTE",
      addressCountry: "FR",
    },
    knowsLanguage: [
      {
        "@type": "Language",
        name: "French",
        alternateName: "fr"
      },
      {
        "@type": "Language",
        name: "English",
        alternateName: "en"
      },
      {
        "@type": "Language",
        name: "Spanish",
        alternateName: "es"
      },
      {
        "@type": "Language",
        name: "Portuguese",
        alternateName: "pt"
      },
    ],
    geo: {
      "@type": "GeoCoordinates",
      latitude: 45,
      longitude: 5,
    },
    areaServed: {
      "@type": "GeoCircle",
      geoMidpoint: {
        "@type": "GeoCoordinates",
        latitude: 45,
        longitude: 5,
      },
      geoRadius: 25000,
    },
    priceRange: "$",
  };
  const ldScript = document.createElement("script");
  ldScript.setAttribute("type", "application/ld+json");
  ldScript.textContent = JSON.stringify(jsonld);
  document.head.appendChild(ldScript);
};
